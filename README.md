# Simulation on Email Phishing

1. Features:
    - [ ] import email addresses of the targets (can be done manually).
    - [ ] generate and email to the targets (can be done manually).
    - [X] capture the clicks.
2. Prerequisite:
    - docker engine, can be verified by `docker --version`
    - docker-compose, can be verified by `docker-compose --version`
2. Installation

   - clone source code
   ```shell
    > git clone git@github.com:SarathLUN/phishing-simulation.git 
    > cd phishing-simulation/
    ``` 
   - start with docker-compose as background services
   ```shell
    > docker-compose up -d
    ```
3. Usage
- when the user click on the link, will redirect to below page
![Screenshot-01.png](./images/Screenshot-01.png)
- the email will be captured in database
![Screenshot-02.png](./images/Screenshot-02.png)
