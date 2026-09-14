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
                    echo "-------------------------------------------------------"
                    echo "STAGE: Setup Environment Variables from Infisical Plugin"
                    echo "-------------------------------------------------------"
                }
                withInfisical(
                    configuration: [
                        infisicalCredentialId: 'infisical-satuarah',
                        infisicalEnvironmentSlug: 'prod',
                        infisicalProjectSlug: 'satuarah-vs-o4',
                        infisicalUrl: 'https://app.infisical.com'
                    ],
                    infisicalSecrets: [
                        infisicalSecret(
                            includeImports: true,
                            path: '/waras',
                            secretValues: [
                                [infisicalKey: 'PORT', isRequired: false],
                                [infisicalKey: 'WHATSAPP_PHONE_NUMBER_ID', isRequired: true],
                                [infisicalKey: 'FACEBOOK_API_TOKEN', isRequired: true],
                                [infisicalKey: 'FACEBOOK_CLIENT_ID', isRequired: false],
                                [infisicalKey: 'FACEBOOK_CLIENT_SECRET', isRequired: false],
                                [infisicalKey: 'WEBHOOK_VERIFY_TOKEN', isRequired: true],
                                [infisicalKey: 'WEBHOOK_FORWARD_URL', isRequired: false],
                                [infisicalKey: 'FORWARD_WEBHOOK_URL', isRequired: false]
                            ]
                        )
                    ]
                ) {
                    sh '''
                        set -eu

                        cat <<EOF > .env
PORT=${PORT:-3000}
WHATSAPP_PHONE_NUMBER_ID=${WHATSAPP_PHONE_NUMBER_ID}
FACEBOOK_API_TOKEN=${FACEBOOK_API_TOKEN}
FACEBOOK_CLIENT_ID=${FACEBOOK_CLIENT_ID:-}
FACEBOOK_CLIENT_SECRET=${FACEBOOK_CLIENT_SECRET:-}
WEBHOOK_VERIFY_TOKEN=${WEBHOOK_VERIFY_TOKEN}
WEBHOOK_FORWARD_URL=${WEBHOOK_FORWARD_URL:-${FORWARD_WEBHOOK_URL:-}}
EOF
                        cp .env .env.production
                        chmod 600 .env .env.production
                        echo "Environment file (.env & .env.production) generated successfully."
                    '''
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
