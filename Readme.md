# Home Assignment for payment backend engineer (Rust)

## Overview

You will build a cli that convert postal address formats and save them in local files (on a json file as a local database). The project should adhere to good architecture principles, ensuring modularity, maintainability, and the ability to extend the application with minimal effort.

The CFONB wrote a recommendation to translate french postal addresses format into ISO 20022 postal adresses format : https://www.cfonb.org/fichiers/20210621111227_Guide_CFONB_Recommandations_Transposition_en_adresse_postale_structuree_ISO__V1.0.pdf.

Once finished, you should share a public repo on GitHub with the code.

## Requirements

- **Internal Model**: Design an internal model that can contain everything needed to build postal addresses in both French format and ISO 20022 format.
- **Clean domain**: You should have all the use cases to convert `from` and `to` the ISO 20022, the french format and the internal model ; and to save / update / delete addresses from a repository.
- **Command-Line Interface (CLI)**: The cli should be a presenter for the domain, exposing it locally.
- **Repository Pattern**: The use cases should be able to interact with any type of repository. We need a repository abstract interface implemented as an in-memory repository to run the uni tests and a file-based repository for integration tests and the actual cli.
- **Extensibility**: The architecture should demonstrate how easy it is to add another presenter (like an API), and another repository (like a database) without changes to the domain. Don't actually implement them, just have a few lines of comments to show where it would be.
- **Testing**:
  - **Unit Tests**: Focus on behavior units and not function units, ensuring the tests validate the expected behavior of the use cases and models.
  - **Integration Tests**: Validate the application using the file repository to ensure it meets the specifications.
- **General Considerations** : Ensure the code is modular, maintainable, and open to future enhancements.

## Evaluation Criteria

- **Understanding of Specifications**: Ability to read and comprehend the provided specifications thoroughly.
- **Autonomy**: Demonstrating independence in problem-solving and decision-making.
- **Quality of Architecture**: Adherence to clean architecture principles, effective use of design patterns, and overall structure. The dependencies should be injected in the right direction.
- **Modularity and Maintainability**: The design should allow for easy modifications and additions in the future.
- **Testing Quality**: Comprehensive testing strategy that ensures reliability and correctness of the application.

- **Bonus point**: if you are used to it, add a GitHub action to test / build / ... whatever you think will impress us. CI / CD

## Workload

If you have already worked on this kind of task, this should be a 2-3 hours workload. If you are new to those principles, it might be longer, but you'll learn things.
