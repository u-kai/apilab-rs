use crate::url_encode::core::UrlEncoder;

#[derive(Clone, Debug)]
pub struct OAuth1Authorizer {
    pub endpoint: String,
    pub oauth_callback: String,
    pub oauth_token: String,
    pub url_encoder: UrlEncoder,
}
impl OAuth1Authorizer {
    //pub async fn fetch_access_token(&self)->
    pub fn parse_authorization_response(
        &self,
        redirect_response: &str,
    ) -> OAuth1AuthorizationResponse {
        let params = redirect_response
            .split('?')
            .skip(1)
            .next()
            .expect(&format!("{} is not redirect_response", redirect_response));
        println!("{:#?}", params);
        let mut params = params
            .split('&')
            .map(|kv| kv.split('=').skip(1).next().unwrap());
        OAuth1AuthorizationResponse {
            oauth_token: params.next().unwrap().to_string(),
            oauth_token_secret: params.next().unwrap().to_string(),
            oauth_verifier: params.next().unwrap().to_string(),
        }
    }
    pub fn authorization_url(&self) -> String {
        format!(
            "{}?oauth_token={}&oauth_callback={}",
            self.endpoint,
            self.url_encoder.encode(&self.oauth_token),
            self.url_encoder.encode(&self.oauth_callback)
        )
    }
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct OAuth1AuthorizationResponse {
    pub oauth_token: String,
    pub oauth_token_secret: String,
    pub oauth_verifier: String,
}

#[cfg(test)]

mod oauth1_authorizer_test {
    use crate::url_encode::core::UrlEncoder;

    use super::*;
    #[test]
    fn authorization_url_test() {
        let oauth1_authorizer = OAuth1Authorizer {
            endpoint: "https://api.twitter.com/oauth/authorize".to_string(),
            url_encoder: UrlEncoder::for_oauth(),
            oauth_callback: "https://127.0.0.1/callback".to_string(),
            oauth_token: "sdf0o9823sjdfsdf".to_string(),
        };
        let url = oauth1_authorizer.authorization_url();
        let tobe ="https://api.twitter.com/oauth/authorize?oauth_token=sdf0o9823sjdfsdf&oauth_callback=https%3A%2F%2F127.0.0.1%2Fcallback";
        assert_eq!(&url, tobe);
    }
    #[test]

    fn parse_authorization_response_test() {
        let authorizer = OAuth1Authorizer {
            endpoint: String::new(),
            oauth_callback: String::new(),
            oauth_token: String::new(),
            url_encoder: UrlEncoder::new(),
        };
        let redirect_response = "https://127.0.0.1/callback?oauth_token=kjerht2309uf&oauth_token_secret=lsdajfh923874&oauth_verifier=w34o8967345";
        let authorization_response = authorizer.parse_authorization_response(redirect_response);
        assert_eq!(
            authorization_response,
            OAuth1AuthorizationResponse {
                oauth_token: "kjerht2309uf".to_string(),
                oauth_token_secret: "lsdajfh923874".to_string(),
                oauth_verifier: "w34o8967345".to_string(),
            }
        );
    }
}
