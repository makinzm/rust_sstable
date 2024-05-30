# How to run

1. Build docker image

```bash
docker build -t myapp .
# If you want to build with a different name, replace `myapp` with your desired name
# If you want to just check image `docker run -it myapp /bin/bash`
```

2. Apply k8s manifests

```bash
kubectl apply -f deployment.yaml
```

3. Check the deployment status

```bash
kubectl describe pod myapp
kubectl get pods

kubectl logs <pod-name>
```

