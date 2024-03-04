# telegram-sender

Greetings! This endeavor served as a learning experience for [Teloxide](https://github.com/teloxide/teloxide), [SurrealDb](https://surrealdb.com/), [Knative](https://knative.dev/docs/), and [Rust](https://www.rust-lang.org/). It involves an event-driven approach for dispatching messages to users through a [Telegram bot](https://core.telegram.org/bots/api) deployed in [Kubernetes](https://kubernetes.io/) with the assistance of [Knative](https://knative.dev/docs/).

### Quick Links

- [Getting Started](#getting-started)
- [Prerequisites](#prerequisites)
- [Installation](#installation)
- [Usage](#usage)
- [Roadmap](#roadmap)
- [License](#license)

## Getting Started

In order to send messages to users, it's necessary to have a database containing user emails and chat IDs. This can be accomplished by utilizing the [telegram-bot](https://github.com/gggiulio77/telegram-bot) repository. Subsequently, we can trigger this sender through a [Cloud Event](https://cloudevents.io/), providing the user email and message for sending. This can be achieved through an HTTP request or any other method offered by [Knative Eventing](https://knative.dev/docs/eventing/).

### Prerequisites

Before proceeding, ensure you have [Rust](https://www.rust-lang.org/tools/install) installed on your system.

For running it locally, you'll need to deploy a [SurrealDb database](https://surrealdb.com/docs/surrealdb/installation/running/docker) and obtain a [Telegram bot token](https://core.telegram.org/bots/tutorial#obtain-your-bot-token).

Alternatively, you will need a [Kubernetes](https://kubernetes.io/) cluster with [Knative installed](https://knative.dev/docs/install/) to run it in a cloud environment.


### Installation

`git clone https://github.com/gggiulio77/telegram-sender.git`

## Usage

For local execution, simply create a `.env` file containing all required information.

For deployment in [Knative](https://knative.dev/docs/), utilize `kubectl apply -f knative.server.service.yml`. Certain environment values are stored in [Kubernetes Secrets](https://kubernetes.io/docs/concepts/configuration/secret/), thus you'll need to refactor the file or create secrets accordingly.

If you have a local image repository on port `32000`, you can utilize the `build.sh` script in the scripts folder to build the project and update `knative.server.service.yml` with the new image digest.

## Roadmap

- [ ] Improve documentation 
- [ ] Include a docker-compose file for local execution
- [ ] Add more messages types

## License