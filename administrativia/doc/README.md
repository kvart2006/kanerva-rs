<div align="center">
        <img width="20%" src="../images/plars.jpg" alt="plars" title="plars"></img>
</div>

# Useful commands for dev use only

## cargo

Execute individual binary:

```
cargo run --bin plars-frontend
```

## keycloak

| Script | Description |
| --- | --- |
| compilekc.sh | compile keycloak |
| startkc.sh | start keycloak |

Keycloak is an integrated SSO and IDM for browser apps and RESTful web services, built on top of OAuth 2.0, OpenID Connect, JSON Web Tokens (JWT) and SAML 2.0 specifications. Keycloak has tight integration with a variety of platforms and has an HTTP security proxy service where we don't have tight integration.

### Starting keycloak from github
```
git clone https://github.com/keycloak/keycloak.git
cd keycloak
mvn install
mvn -f testsuite/utils/pom.xml exec:java -Pkeycloak-server 
```

### Starting standalone keycloak server
```
cd keycloak
bin/standalone.sh
```

### Connecting to keycloak
After the server boots, open your browser and go to the
```
http://localhost:8080/auth
```

### Starting keycloak proxy

The keycloak proxy distribution is located in the `kc` directory. Use the provided `plars.json` configuration file as
```
kc/keycloak-proxy-3.4.3.Final/bin/launcher.jar kc/plars.json
```


Obtain access token for the admin in the realm master with username `admin` and password `admin`:
```
curl -d "client_id=admin-cli" -d "username=admin" -d "password=admin" -d "grant_type=password"   "http://localhost:8081/auth/realms/master/protocol/openid-connect/token"
```

## git

```
git checkout --orphan latest_branch && git add -A && git commit -am "commit message" && git branch -D master && git branch -m master && git push -f origin master
```

### Authors
Bela Berde, Ph.D., 
<a href="mailto:bela.berde@nokia-bell-labs.com">bela.berde@nokia-bell-labs.com</a>

<footer><small>&copy; Copyright 2018, Nokia Bell Labs France</small></footer