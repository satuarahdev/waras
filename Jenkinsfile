pipeline {
    agent any

    options {
        timestamps()
        disableConcurrentBuilds()
        buildDiscarder(logRotator(numToKeepStr: '20'))
        skipDefaultCheckout(true)
        timeout(time: 30, unit: 'MINUTES')
    }

    environment {
        COMPOSE_FILE = 'docker-compose.yml'
        COMPOSE_PROJECT_NAME = 'waras-whatsapp-bot'
        CONTAINER_NAME = 'waras'
    }

    stages {
        stage('Checkout') {
            steps {
                deleteDir()
                checkout scm
            }
        }

        stage('Prepare Environment') {
            steps {
                script {
                    withCredentials([file(credentialsId: 'env-api-waras.satuarah.id', variable: 'JENKINS_ENV_FILE')]) {
                        sh '''
                            set -eu
                            cp "$JENKINS_ENV_FILE" .env
                            cp "$JENKINS_ENV_FILE" .env.production
                            chmod 600 .env .env.production
                            
                            echo "=== DEBUG: Isi file .env yang ditarik Jenkins ==="
                            cat .env || true
                            echo "================================================="
                        '''
                    }
                }
            }
        }

        stage('Build & Deploy') {
            steps {
                sh '''
                    set -eu
                    docker compose -p ${COMPOSE_PROJECT_NAME} -f ${COMPOSE_FILE} build
                    docker rm -f ${CONTAINER_NAME} || true
                    docker compose -p ${COMPOSE_PROJECT_NAME} -f ${COMPOSE_FILE} up -d --force-recreate --remove-orphans
                '''
            }
        }

        stage('Health Check') {
            steps {
                sh '''
                    set -eu
                    sleep 5
                    if ! docker ps --format '{{.Names}}' | grep -q "^${CONTAINER_NAME}\\$"; then
                        echo "Container ${CONTAINER_NAME} is not running!"
                        docker logs ${CONTAINER_NAME} || true
                        exit 1
                    fi
                '''
            }
        }
    }
}
