# Django Build

Django build is a CLI tool that was created to enhance and advance the capabilities the django-admin cli tool provides in creating, configuring and managing Django projects.
The `django-admin` tool offers an easy way to start a django project by running, `django-admin startproject <project_name>`.
This creates:
1. A new directory with the name of your project.
2. Another directory inside the main one which holds your project files

These files comprises of:
1. asgi.py
2. wsgi.py
3. urls.py
4. settings.py
5. __init__.py

They came with basic configurations. The Django build CLI builds on top of that. It not only creates the Django project but sets up your project with all of the tools and configurations you'll  
need so that you can focus less on setting up your environment and installing packages and configuring urls and more on actually writting your code.

## Commands in django-build  
1. Start a Project
   `django-build project create <file_name>` - This creates a couple of things:
   a.  Django project.
   b. Creates a docker and docker-compose file
   c. Creates a virtual environment where you can run your application.
   d. Creates a requirement.txt file that has list of packages that are mostly and frequently used when building Web API's.
   e. Creates .env file for your environment variables
   f. The settings.py comes configured with the python packages such as `djangorestframework`, `django-filters`, `simple-jwt` , `drf-spectacular`, `corsheaders`, etc and their corresponding configurations.
   g. It also comes with a django app calles `users` and a customized **User Model** without forgetting **urls.py, filters.py, and serializers.py**.
   
