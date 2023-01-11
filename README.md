# Zero to potentially back end rust hero


## one: What is this project?

This is a project to enable to me learn as much as I can about back end rust development, building essentially a production grade project. 
I've done the programming challenge exercises, I done some leetcode, now I should get on project buidling with intention. 

At each step, comparing different tools for the job, and understanding deeper requirements.

## two: So what are the requirements here

I will be building up to a newsletter bakc end api with suscription functionalites, scalabibilty, CI / CD, Test Driven etc. 

Requirements:
- As a blog visitor,
I want to subscribe to the newsletter,
So that I can receive email updates when new content is published on the blog.

- As a blog author,
I want to send an email to all my subscribers,
So that I can notify them when new content is published.

## three: signing up a new subscriber

Project at the moment is empty:
Need to decide on a project, framework, database, testing strategy, and some endpoint setup.

### CI Pipeline
Using github actions to automate builds on prs or pushes.

### Framework: Actix-web
Chosen because of the popular use of this tool, community is important, and documentation so far is pretty good.
I have tried out Rocket in the past, but didn't gel with me.

#### Database: SQLX
Written using general SQL syntax. So it is pretty portable and resistant to if I want to swap out the framework at a later date.
How to use, is pretty similar to other tools such as Diesel. Diesel might be worth looking at later, but requires some additional learning.

## four: Telemetary
Telemetary allows us to be aware of the unknown.
Tests are good, but we need logs in production that would allow us to identify and be able to have enough information to debug an issue.

any interaction with external systems over the network should be closely monitored.

As a rule of thumb, everything we use in our application should be reflected in our integration tests.
 
To add telemetary is to instrument the application.
if you cannot debug it from logs, imagine how difficult would it be to debug in production!

## five: Deployment

We are now up for deploying to a cloud provider to host this newsletter api.

Whats involved is ensuring prod, local environment variables, as well as setting up runtime environment variables.
We have chosen to use digital ocean for deployment, although any cloud provider should suffice. Digital ocean eases the process of generating https certificates.
Digital ocean setup is done via yaml specs that allow you to define features, environment variables, ci webhook to github repo, postgres database provisioning etc.

# six: static email validation


