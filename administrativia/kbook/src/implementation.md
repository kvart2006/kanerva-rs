## MSP Implementation

### API-first microservice platform





### Addressing
| platform | address | api | IP |
| --- | --- | --- | --- |
| plars | <a href="https://dev.plars.com">https://dev.plars.com</a> | <a href="https://api.plars.com">https://api.plars.com</a> | 127.0.0.1:8000 |
| business layer | <a href="https://dev.plars-bl.com">https://dev.plars-bl.com</a> | <a href="https://api.plars-bl.com"> https://api.plars-bl.com</a> | 127.0.0.1:8001 |
| design layer | <a href="https://dev.plars-dl.com">https://dev.plars-dl.com</a> | <a href="https://api.plars-dl.com"> https://api.plars-dl.com</a> | 127.0.0.1:8002 |
| operation layer | <a href="https://dev.plars-ol.com">https://dev.plars-ol.com</a> | <a href="https://api.plars-ol.com">https://api.plars-ol.com</a> | 127.0.0.1:8003 |


### WebServer basics

A web server is implemented first using a request handler.
A request handler is a function that accepts a ```HttpRequest``` instance as its only parameter and returns a type that can be converted into ```HttpResponse```:

    fn index(req: HttpRequest) -> &'static str 
    {
          "Hello world!"
    }

Next, an ```Application``` instance is created that registers the request handler with the application's resource on a particular HTTP method and path::

    Application::new().resource("/", |r| r.f(index));

After that, application instance can be used with ```HttpServer``` to listen for incoming connections. Server accepts function that should return ```HttpHandler``` instance:

    HttpServer::new(|| Application::new().resource("/", |r| r.f(index)))
       .bind("127.0.0.1:8088")?
       .run();



<footer><small>&copy; Copyright 2018, Nokia Bell Labs France</small></footer>


