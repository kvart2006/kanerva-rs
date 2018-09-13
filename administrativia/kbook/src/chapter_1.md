## smers architecture

1 Soft/hardware-powered 
Leverage both software and hardware network components in the architecture
2 Businesswide fabric 
Recognize that the digital business network is a fabric connecting every interrelated part of the business.
3 Interwoven layers 
Enable a vertical interaction of layer 2 through layer 7 throughout the network.
4 Automated actions 
Build out a network automation and orchestration system that is programmable.
5 Empowered users
Create a network that empowers others, especially customers, to use the network as needed.

### Platform

Platform: a place or opportunity for public discussion.

Platforms are commonly defined as systems that match adopters of the system with firms that provide complements to the system. [1]

[1] Platform Revolution: How Networked Markets Are Transforming the Economy - and How to Make Them Work for You, Geoffrey G. Parker, Marshall W. Van Alstyne, Sangeet Paul Choudary, W. W. Norton & Company, 2016.

Open source projects take many forms:
- Components: Projects that address a narrowly defined problem whose output may be
consumed as an atomic entity. Examples: OpenvSwitch (OVS, virtual switch), a platform
plug-in to integrate new hardware or software.
- Platforms: Projects whose scope encompasses multiple components to yield a
framework that can be adapted to meet a range of different user needs. Examples:
OpenDaylight (SDN Controller Framework; Open Network Automation Platform (ONAP)
open orchestration framework); and OpenStack (NFV Virtualized Infrastructure Manager).
- Open Reference Platforms: Projects that focus on the integration of platforms and
components, and are primarily used to test, demonstrate, and validate broader solutions.
Examples: OPNFV NFV reference platform and MEF OpenLSO reference platform.


A platform is a communication-based operating environment, which provides reusable capabilities for a particular purpose aligned with the core competences strategy of the platform owner. 
Since core competence cannot remain static, building a platform today means adopting an evolutionary architecture [1]. 
A platform then uses a cloud-native infrastructure, which abstracts some aspects of the physical infrastructure in a managed way by its own, with the goal of making possible the interaction with cloud-native applications. 

A platform is often called x-as-a-Service. These include Dashboard-as-a-Service (DaaS), Network as a Service (NaaS), Communications-as-a-Service (CaaS) such as voice over IP (VoIP or Internet telephony) etc.






With the abstractions in place, the platform enables cloud-native applications to be built, ship, and run.  

In cloud-native architecture building, the evolvability in many dimensions implies using microservices for architecture modularity. Microservice goes beyond the cloud-native concept, however, since it allows not only composition of microservices but also their replacement. Further, the requirement for almost-infinite scaling brings on 



API management systems have become a monolith and source of friction for clients" augh enterprise flashbacks to endless cycles of api management choice


### In Depth: Layering a Shipping System

Let’s look at the implications of applying RESPONSIBILITY LAYERS to
the cargo shipping application discussed in the examples of previous
chapters.

As we rejoin the story, the team has made considerable progress
creating a MODEL-DRIVEN DESIGN and distilling a CORE DOMAIN.
But as the design fleshes out, they are having trouble coordinating
how all the parts fit together. They are looking for a large-scale structure
that can bring out the main themes of their system and keep
everyone on the same page.

The team members have been steeped in the domain of shipping
for months, and they have noticed some natural stratification of its
concepts. It is quite reasonable to discuss transport schedules (the
scheduled voyages of ships and trains) without referring to the cargoes
aboard those transports. It is harder to talk about tracking a
cargo without referring to the transport carrying it. The conceptual
dependencies are pretty clear. The team can readily distinguish two
layers: “Operations” and the substrate of those operations, which
they dub “Capability.”

### “Operational” Responsibilities
Activities of the company, past, current, and planned, are collected
into the Operations layer. The most obvious Operations object is
Cargo, which is the focus of most of the day-to-day activity of the
company. The Route Specification is an integral part of Cargo, indicating
delivery requirements. The Itinerary is the operational delivery
plan. Both of these objects are part of the Cargo’s AGGREGATE,
and their life cycles are tied to the time frame of an active delivery.

### “Capability” Responsibilities

This layer reflects the resources the company draws upon in order to
carry out operations. The Transit Leg is a classic example. The ships
are scheduled to run and have a certain capacity to carry cargo,
which may or may not be fully utilized.
True, if we were focused on operating a shipping fleet, Transit
Leg would be in the Operations layer. But the users of this system
aren’t worried about that problem. (If the company were involved in
both those activities and wanted the two coordinated, the development
team might have to consider a different layering scheme, perhaps
with two distinct layers, such as “Transport Operations” and
“Cargo Operations.”)

A trickier decision is where to place Customer. In some businesses,
customers tend to be transient: they’re interesting while a
package is being delivered and then mostly forgotten until next time.
This quality would make customers only an operational concern for a
parcel delivery service aimed at individual consumers. But our hypothetical
shipping company tends to cultivate long-term relationships
with customers, and most work comes from repeat business. Given
these intentions of the business users, the Customer belongs in the potential
layer. As you can see, this was not a technical decision. It was
an attempt to capture and communicate knowledge of the domain.
Because the association between Cargo and Customer can be
traversed in only one direction, the Cargo REPOSITORY will need a
query that finds all Cargoes for a particular Customer. There were
good reasons to design it that way anyway, but with the imposition of
the large-scale structure, it is now a requirement.


While the distinction between Operations and Capability clarifies
the picture, order continues to evolve. After a few weeks of experimentation,
the team zeroes in on another distinction. For the
most part, both initial layers focus on situations or plans as they are.
But the Router (and many other elements excluded from this example)
isn’t part of current operational realities or plans. It helps make
decisions about changing those plans. The team defines a new layer
responsible for “Decision Support.”

### “Decision Support” Responsibilities

This layer of the software provides the user with tools for planning
and decision making, and it could potentially automate some decisions
(such as automatically rerouting Cargoes when a transport
schedule changes).

The Router is a SERVICE that helps a booking agent choose the
best way to send a Cargo. This places the Router squarely in Decision
Support.

The references within this model are all consistent with the three
layers except for one discordant element: the “is preferred” attribute
on Transport Leg. This attribute exists because the company prefers
to use its own ships when it can, or the ships of certain other companies
with which it has favorable contracts. The “is preferred” attribute
is used to bias the Router toward these favored transports. This
attribute has nothing to do with “Capability.” It is a policy that directs
decision making. To use the new RESPONSIBILITY LAYERS, the
model will have to be refactored.


### RFB

Structural patterns are concerned with how objects are composed to form larger structures.
The key to the composite pattern is an abstract class that represents both
primitives and their containers.
<img src="images/RFB-microservice.jpg" alt="RFB" width="200" class="center" />
<p style="text-align:center">Figure 1: Microservice and RFB.</p>



Cloud-native infrastructure is a requirement to effectively run cloud-native applications.

Developers write application code and define the application dependencies, and it is the platform’s responsibility to create the necessary infrastructure to run, manage, and expose
it. Unlike IaaS, which still requires infrastructure management, in a PaaS the
infrastructure is managed by the platform provider.

It turns out, PaaS limitations required developers to write their applications
differently to be effectively managed by the platform. Applications had to
include features that allowed them to be managed by the platform without access
to the underlying operating system. The application’s life cycle and
management were now controlled by the PaaS, and engineers and applications
needed to adapt.

Application development cycles were reduced because engineers did not need to spend time managing infrastructure.
Applications that embraced running on a platform were the beginning of what
we now call “cloud native applications.” They exploited the platform limitations
in their code and in many cases changed how applications are written today.

However, many PaaS platforms are not enough for everything a business needs.
They often limit language runtimes, libraries, and features to meet their promise
of abstracting away the infrastructure from the application. Public PaaS
providers will also limit which services can integrate with the applications and
where those applications can run.

Cloud native infrastructure is infrastructure that is hidden behind useful
abstractions, controlled by APIs, managed by software, and has the purpose of
running applications. Running infrastructure with these traits gives rise to a new
pattern for managing that infrastructure in a scalable, efficient way. Cloud native infrastructure needs to abstract the underlying IaaS offerings to
provide its own abstractions. The new layer is responsible for controlling the
IaaS below it as well as exposing its own APIs to be controlled by a consumer.


Cloud-native is not about microservices or infrastructure as code. Microservices
enable faster development cycles on smaller distinct functions, but monolithic
applications can have the same features that enable them to be managed
effectively by software and can also benefit from cloud native infrastructure.

Infrastructure as code defines and automates your infrastructure in machine-parsible
language or domain-specific language (DSL). Traditional tools to apply
code to infrastructure include configuration management tools (e.g., Chef and
Puppet). These tools help greatly in automating tasks and providing consistency,
but they fall short in providing the necessary abstractions to describe
infrastructure beyond a single server.

Configuration management tools automate one server at a time and depend on
humans to tie together the functionality provided by the servers. This positions
humans as a potential bottleneck for infrastructure scale. These tools also don’t
automate the extra parts of cloud infrastructure (e.g., storage and network) that
are needed to make a complete system.

While configuration management tools provide some abstractions for an
operating system’s resources (e.g., package managers), they do not abstract away
enough of the underlying OS to easily manage it. If an engineer wanted to
manage every package and file on a system, it would be a very painstaking
process and unique to every configuration variant. Likewise, configuration
management that defines no, or incorrect, resources is only consuming system
resources and providing no value.

Just as the cloud changed the relationship between business and infrastructure,
cloud-native applications changed the relationship between applications and
infrastructure. We need to see what is different about cloud-native compared to
traditional applications so we can understand their new relationship with
infrastructure.

Cloud-native applications acquire these traits through various methods. It can
often depend on where your applications run and the processes and culture of
the business. The following are common ways to implement the desired
characteristics of a cloud-native application:
- Microservices
- Health reporting
- Telemetry data
- Resiliency
- Declarative, not reactive


How Do Cloud-native Applications Impact Infrastructure?

Hopefully, you can tell that cloud-native applications are different than
traditional applications. Cloud native applications do not benefit from running
directly on IaaS or being tightly coupled to a server’s operating system. They
expect to be run in a dynamic environment with mostly autonomous systems.
Cloud native infrastructure creates a platform on top of IaaS that provides
autonomous application management. The platform is built on top of
dynamically created infrastructure to abstract away individual servers and
promote dynamic resource allocation scheduling.

Applications with these characteristics need a platform that can pragmatically
monitor, gather metrics, and then react when failures occur. Cloud native
applications do not rely on humans to set up ping checks or create syslog rules.
They require self-service resources abstracted away from selecting a base
operating system or package manager, and they rely on service discovery and
robust network communication to provide a feature-rich experience.


The infrastructure required to run cloud-native applications is different than
traditional applications. Many responsibilities that infrastructure used to handle
have moved into the applications.
Cloud native applications simplify their code complexity by decomposing into
smaller services. These services provide monitoring, metrics, and resiliency built
directly into the application. New tooling is required to automate the
management of service proliferation and life cycle management.
The infrastructure is now responsible for holistic resource management, dynamic
orchestration, service discovery, and much more. It needs to provide a platform
where services don not rely on individual components, but rather on APIs and
autonomous systems.

Applications are the easiest part to get ready. The design patterns are well
established, and tooling has improved dramatically since the advent of the public
cloud. If you are not able to build cloud native applications and automatically
deploy them through a verified and tested pipeline, you should not move forward
with adopting the infrastructure to support them.
Building cloud native applications does not mean you must first have
microservices. It does not mean you have to be developing all your software in
the newest trending languages. It means you have to write software that can be
managed by software.

### Applications

The only interaction humans should have with cloud-native applications is
during their development. Everything else should be managed by the
infrastructure or other applications.
Another way to know applications are ready is when they need to dynamically
scale with multiple instances. Scaling typically implies multiple copies of the
same application behind a load balancer. It assumes that applications store state
in a storage service (i.e., database) and do not require complex coordination
between running instances.

Dynamic application management implies that a human is not doing the work.
Application metrics trigger the scaling, and the infrastructure does the right thing
to scale the application. This is a basic feature of most cloud environments.
Running autoscaling groups doesn’t mean you have cloud native infrastructure;
but if auto-scaling is a requirement, it may indicate that your applications are
ready.

In order for applications to benefit, the people who write the applications and
configure the infrastructure need to support this method of working. Without
people ready to give up control to software, you’ll never realize the benefits.

### Systems

Cloud native applications need system abstractions. The application should not
be concerned with an individual, hardcoded hostname. If your applications
cannot run without careful placement on individual hosts, then your systems are
not ready for cloud native infrastructure.

Taking a single server (virtual or physical) running an operating system and
turning it into a method by which to access resources is what we mean when we
say “abstractions.” Individual systems should not be the target of deployment for
an application. Resources (CPU, RAM, and disk) should be pooled across all
available machines and then divvied up by the platform from applications’
requests.

In cloud native infrastructure, you must hide underlying systems to improve
reliability. Cloud infrastructure, like applications, expects failures of underlying
components to occur and is designed to handle such failures gracefully. This is
needed because the infrastructure engineers no longer have control of everything
in the stack.

Infrastructure is ready to become cloud native when it is no longer a challenge.
Once infrastructure becomes easy, automated, self-serviceable, and dynamic, it
has the potential to be ignored. When systems can be ignored and the technology
becomes mundane, it’s time to move up the stack.

### Business

If the architecture of the system and the architecture of the organization are at
odds, the architecture of the organization wins.
Ruth Malan, “Conway’s Law”
Businesses are very slow to change. They may be ready to adopt cloud native
practices when scaling people to manage scaling systems is no longer working,
and when product development requires more agility.
People don’t scale infinitely. For each person added to manage more servers or
develop more code, there is a strain on the human infrastructure that supports
them (e.g., office space). There is also overhead for other people because there
needs to be more communication and more coordination.
As we discussed in Chapter 1, by using a public cloud, you can reduce some of
the process and people overhead by renting server time. Even with a public
cloud, you will still have people that manage infrastructure details (e.g., servers,
services, and user accounts).
The business is ready to adopt cloud native practices when the communication
structure reflects the infrastructure and applications the business needs to create.
This includes communications structures that mirror architectures like
microservices. They could be small, independent teams that do not have to go
through layers of management to talk to or work with other teams.

The raw number of deploys does not matter. What matters is providing customer
value as quickly as possible. Believing the software deployed will meet all of the
customer’s needs the first time, or even the 100th time, is a fallacy.
When the business realizes it needs to iterate and change frequently, it may be
ready to adopt cloud native applications. As soon as it finds limitations in people
efficiency and old process limitations, and it’s open to change, it is ready for
cloud native infrastructure.

All the factors that indicate when to adopt cloud native don’t tell the full story.
Any design is about trade-offs. So here are some situations when cloud native
infrastructure is not the right choice.

### Infrastructure as Software

Infrastructure as code was a powerful move in the right direction. But code is a
static representation of infrastructure and has limitations. You can automate the
process of deploying code changes, but unless the deployment tool runs
continually, there will still be configuration drift. Deployment tooling
traditionally works only in a single direction: it can only create new objects, and
can’t easily delete or modify existing objects.

To master infrastructure, our deployment tools need to work from the initial
representation of infrastructure, and mutate the data to make more agile systems.
As we begin to look at our infrastructure representation as a versionable body of
data that continually enforces the desired state, we need the next step of
infrastructure as software.

IaaS presented raw components as provisionable API endpoints, and platforms
present APIs for resources that are more easily consumed by applications. Some
of those resources may provision IaaS components (e.g., load balancers or disk
volumes), but many of them will be be managed by the platform (e.g., compute
resources).

Platforms expose a new layer of infrastructure and continually enforce the
desired state. The components of the platforms are also applications themselves
that can be managed with the same desired state declarations.

The API machinery allows users to reap the benefits of standardizing
infrastructure as code, and adds the ability to version and change the
representation over time. APIs allow a new way of consuming resources through
standard practices such as API versioning. Consumers of the API can build their
applications to a specific version and trust that their usage will not break until
they choose to consume a new API version. Some of these practices are critical
features missing from previous infrastructure as code tools.

Encapsulating infrastructure and thinking of it as a versioned API is remarkably
powerful. This dramatically increases velocity of a software project responsible
for interpreting a representation. Abstractions provided by a platform are
necessary to keep up with the quickly growing cloud. This new pattern is the
pattern of today, and the one that has been proven to scale to unfathomable
numbers for both infrastructure and applications.

In regard to the directions, infrastructure as code typically is read-only (like Terraform) whereas infrastructure as software is a negotiation between the software that is running and the declared state a user intended. Infrastructure as software often will mutate. 

### Idempotency

Software can be idempotent, meaning you must be able to continually feed it the
same input, and always get the same output.
In technology, this idea was made famous by the Hypertext Transfer Protocol
(HTTP) via idempotent methods like PUT and DELETE. This is a very powerful
idea, and advertising the guarantee of idempotency in software can drastically
shape complex software applications.

One of the lessons learned in early configuration management tools was
idempotency. We need to remember the value this feature offered infrastructure
engineers, and continue to build this paradigm into our tooling.
Being able to automatically create, update, or delete infrastructure with the
guarantee that no matter how often you run the task it will always output the
same is quite exciting. It allows for operators to begin to work at automating
tasks and chores. What used to be a sizable amount of work for an operator can
now be as simple as a button click in a web page.

The idempotent guarantee is also effective at helping operators perform quality
science on their infrastructure. Operators could begin replicating infrastructure in
many physical locations, with the knowledge that someone else repeating their
procedure would get the same thing.
We began to notice entire frameworks and toolchains built around this idea of
automating arbitrary tasks for repeatability.

As it was with software, so it became with infrastructure. Operators began
automating entire pipelines of managing infrastructure using these
representations and deployment tools. The work of an operator now became
developing the tooling around automating these tasks, and no longer performing
the tasks themselves.

### Conclusion

The infrastructure layers of the stack have astonishingly similar histories to the
software application layers. Cloud native infrastructure is no different. We begin
to find ourselves repeating history, and learning age-old lessons in new guises.
What is to be said about the ability to predict the future of the infrastructure
industry if we already know the future of its software counterpart?

Cloud native infrastructure is a natural, and possibly expected, evolution of
infrastructure. Being able to deploy, represent, and manage it in reliable and
repeatable ways is a necessity. Being able to evolve our deployment tools over
time, and shift our paradigms of how this is done, is critical to keeping our
infrastructure in a space that can keep up with supporting its application-layer
counterpart.


In the previous chapter we learned about representing infrastructure and the
various approaches and concerns with deployment tools around it. In this chapter
we look at what it takes to design applications that deploy and manage
infrastructure. We heed the concerns of the previous chapter and focus on
opening up the world of infrastructure as software, sometimes called
infrastructure as an application.

In a cloud native environment, traditional infrastructure operators need to be
infrastructure software engineers. It is still an emerging practice and differs from
other operational roles in the past. We desperately need to begin exploring
patterns and setting standards.

A fundamental difference between infrastructure as code and infrastructure as
software is that software continually runs and will create or mutate infrastructure
based on the reconciler pattern, which we will explain later in this chapter.
Furthermore, the new paradigm behind infrastructure as software is that the
software now has a more traditional relationship with the data store and exposes
an API for defining desired state. For instance, the software might mutate the
representation of infrastructure as needed in the data store, and very well could
manage the data store itself! Desired state changes to reconcile are sent to the
software via the API instead of static code repo.

In the previous chapter we learned about representing infrastructure and the
various approaches and concerns with deployment tools around it. In this chapter
we look at what it takes to design applications that deploy and manage
infrastructure. We heed the concerns of the previous chapter and focus on
opening up the world of infrastructure as software, sometimes called
infrastructure as an application.

In a cloud native environment, traditional infrastructure operators need to be
infrastructure software engineers. It is still an emerging practice and differs from
other operational roles in the past. We desperately need to begin exploring
patterns and setting standards.

A fundamental difference between infrastructure as code and infrastructure as
software is that software continually runs and will create or mutate infrastructure
based on the reconciler pattern, which we will explain later in this chapter.
Furthermore, the new paradigm behind infrastructure as software is that the
software now has a more traditional relationship with the data store and exposes
an API for defining desired state. For instance, the software might mutate the
representation of infrastructure as needed in the data store, and very well could
manage the data store itself! Desired state changes to reconcile are sent to the
software via the API instead of static code repo.
The first step in the direction of infrastructure as software is for infrastructure
operators to realize they are software engineers. We welcome you all warmly to
the field! Previous tools (e.g., configuration management) had similar goals to
change infrastructure operators’ job function, but often the operators only
learned how to write a limited DSL with narrow scope application (i.e., single
node abstraction).

As an infrastructure engineer, you are tasked not only with having a mastery of
the underlying principals of designing, managing, and operating infrastructure,
but also with taking your expertise and encapsulating it in the form of a rocksolid
application. These applications represent the infrastructure that we will be
managing and mutating.

Engineering software to manage infrastructure is not an easy undertaking. We
have all the major problems and concerns of a traditional application, and we are
developing in an awkward space. It’s awkward in the sense that infrastructure
engineering is an almost ridiculous task of building software to deploy
infrastructure so that you can then run the same software on top of the newly
created infrastructure.

To begin, we need to understand the nuances of engineering software in this new
space. We will look at patterns proven in the cloud native community to
understand the importance of writing clean and logical code in our applications.
But first, where does infrastructure come from?


### The API

In earlier chapters we discussed the various methods for representing
infrastructure. In this chapter we will be exploring the concept of having an API
for infrastructure.

When the API is implemented in software, it more than likely will be done via a
data structure. So, depending on the programming language you are using, it’s
safe to think of the API as a class, dictionary, array, object, or struct.
The API will be an arbitrary definition of data values, maybe a handful of
strings, a few integers, and a boolean. The API will be encoded and decoded
from some sort of encoding standing like JSON or YAML, or might even be
stored in a database.

Having a versionable API for a program is a common practice for most software
engineers. This allows the program to move, change, and grow over time.
Engineers can advertise to support older API versions, and offer backwardcompatibility
guarantees. In engineering infrastructure as software, using an API
is preferred for these reasons.

Finding an API as the interface for infrastructure is one of the many clues that a
user will be working with infrastructure as software. Traditionally, infrastructure
as code is a direct representation of the infrastructure a user will be managing,
whereas an API might be an abstraction on top of the exact underlying resources
being managed.1

Ultimately, an API is just a data structure that represents infrastructure.


## Developing Infrastructure Applications

When building applications to manage infrastructure, you need to consider what
APIs you will expose as much as what applications you will create. The APIs
will represent the abstractions for your infrastructure, while the applications
provide and consume APIs in the infrastructure.

It is important to have a firm grasp on why both are important and how you can
use them to your advantage in creating scalable, resilient infrastructure.
In this chapter we will give a fictional example of a cloud native application and
API that go through normal cycles for an application.

### Designing an API

Evolving infrastructure requires evolving the applications that support the
infrastructure. The feature set for these applications will change over time, and
thus infrastructure will implicitly evolve. As infrastructure continues to evolve,
so must the applications that manage it.
Features, needs, and new advances in infrastructure will never stop. If we’re
lucky, the cloud provider APIs will be stable and not change frequently. As
infrastructure engineers, we need to be prepared to react appropriately to these
needs. We need to be ready to evolve our infrastructure and the applications that
support it.

We must create applications that can be scaled and also be ready to scale them.
In order to do this, we need to understand the nuances of making large changes
to our applications without breaking the existing flow of the application.
The beauty of engineering applications that manage infrastructure is that it
liberates the operator from the opinions of others.
The abstractions used in an application are now up to the engineer to craft. If an
API needs to be more literal, it can be; or if it needs to be opinionated and
heavily abstracted, it can be. Powerful combinations of literal and abstracted
definitions can give operators exactly what they want and need for managing
infrastructure.

### Adding Features

Adding a feature to an infrastructure application could be very simple or quite
complex, depending on the nature of the feature. The goal of adding a feature is
that we should be able to add new functionality without jeopardizing existing
functionality. We never want to introduce a feature that will impact other
components of the system in a negative way. Furthermore, we always want to
make sure input into the system remains valid for a reasonable amount of time.

The permutations of infrastructure applications compared to infrastructure APIs
are endless. This offers an extremely flexible and scalable solution for
infrastructure engineers hoping to master their infrastructure in different
environments and in different ways.

The various applications we might build in order to satisfy infrastructure
requirements now become the representation of infrastructure itself. 

It is important to remember that the applications we have been building are
within themselves cloud native applications. This is an interesting twist in the
story, as we are building cloud native applications to manage cloud native
infrastructure.




<footer><small>&copy; Copyright 2018, Nokia Bell Labs France</small></footer>


