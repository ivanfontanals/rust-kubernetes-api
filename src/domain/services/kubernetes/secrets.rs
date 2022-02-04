use crate::domain::model::SecretDto;
use crate::domain::model::SecretRequestDto;
use crate::domain::ports::incoming::SecretService;
use crate::domain::ports::outgoing::Repository;
use crate::domain::ports::outgoing::SealedSecretClient;
use crate::domain::ports::outgoing::VersionControl;

pub struct DefaultSecretsService<R, S, V>
where
  R: Repository<SecretDto>,
  S: SealedSecretClient,
  V: VersionControl,
{
  repository: R,
  sealed_secret_client: S,
  gitops_service: V,
}

impl<R, S, V> DefaultSecretsService<R, S, V>
where
  R: Repository<SecretDto>,
  S: SealedSecretClient,
  V: VersionControl,
{
  pub fn new(repository: R, sealed_secret_client: S, gitops_service: V) -> Self {
    Self {
      repository,
      sealed_secret_client,
      gitops_service,
    }
  }
}

impl<R, S, V> SecretService for DefaultSecretsService<R, S, V>
where
  R: Repository<SecretDto>,
  S: SealedSecretClient,
  V: VersionControl,
{
  fn get(&self, name: &str) -> Option<SecretDto> {
    self.repository.find_by(name)
  }

  fn list(&self) -> Option<Vec<SecretDto>> {
    self.repository.find_all()
  }

  fn create(&self, request: &SecretRequestDto) -> Result<(), anyhow::Error> {
    self
      .gitops_service
      .clone_repo(None)
      .and_then(|gitops_path| {
        let destination_path = format!(
          "{}/infrastructure/_catalog/templates/sealed-secret-{}.yaml",
          gitops_path, request.name
        );
        self.sealed_secret_client.save(request, Some(destination_path)).map(|_| gitops_path)
      })
      .and_then(|gitops_path| match request.skip_pull_request {
        true => self
          .gitops_service
          .auto_commit(gitops_path, "Directly commited a secret from Rust back-end".into()),
        false => self.gitops_service.pull_request(
          gitops_path,
          "Added a new secret from Rust back-end".into(),
          format!("Added secret {} from Rust back-end", request.name),
          "Body of the PR".into(),
          format!("add-secret-{}", request.name),
        ),
      })
  }

  fn render(&self, request: &SecretRequestDto) -> Result<String, anyhow::Error> {
    self.sealed_secret_client.render(request)
  }
}

/*
   SEaled secrets
/// echo -n bar | kubectl create secret generic mysecret --dry-run=client --from-file=foo=/dev/stdin -o json >mysecret.json
/// kubeseal -o yaml <mysecret.yaml >mysealedsecret.yaml
/// kubectl apply -f mysealedsecret.yaml
///
/// export API_KEY=XXXXXXXXXXXXXXXXX
/// echo -n  | kubectl create secret generic mysecret --dry-run --from-file=API_KEY=/dev/stdin -o json | kubeseal -o yaml | grep API_KEY
/// /// Kubeseal installation --> https://github.mpi-internal.com/scmspain/platform-common--schip-secrets/blob/870a1403d51eb1450465516a525e089b485bfeab/.travis.yml
///
Create secret from file
/// echo -n My_API_KEY | kubectl create secret generic mysecret --dry-run --from-file=API_KEY=/dev/stdin -o json | kubeseal -o yaml
/// kubectl create secret generic ssh-key-secret --from-file=ssh-privatekey=/path/to/.ssh/id_rsa --from-file=ssh-publickey=/path/to/.ssh/id_rsa.pub
Create secret from literals
kubectl create secret generic test-db-secret --dry-run --from-literal=username=testuser --from-literal=password=iluvtests -o json | kubeseal -o yaml
*/
