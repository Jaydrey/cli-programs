from rest_framework import serializers
from django.contrib.auth import authenticate

from .models import User


class UserSerializer(serializers.ModelSerializer):
    class Meta:
        model = User
        fields = (
            "id",
            "first_name",
            "last_name",
            "email",
            "profile_picture",
            "date_of_birth",
            "gender",
            "role",
        )
        read_only_fields = ("created_at", "id", "last_login")


class CreateUserSerializer(serializers.ModelSerializer):

    class Meta:
        model = User
        fields = (
            "first_name",
            "last_name",
            "email",
            "password",
        )
        write_only_fields = (
            "password",
        )
        read_only_fields = (
            "id",
            "last_login",
            "is_active",
            "is_superuser",
            "is_staff",
            "is_deleted",
            "created_at",
            "updated_at",
            "groups",
            "user_permissions"
        )

    def validate(self, data: dict):
        try:
            first_name = data['first_name']
            last_name = data['last_name']
            email = data['email']
            password = data['password']
        except Exception as _:
            raise Exception(
                {"error_message": "provide all the necessary fields", "status_code": 400})


        return data

    def create(self, validated_data: dict):
        user: User | None = User.objects.create_user(**validated_data)
        return user


class UserLoginSerializer(serializers.Serializer):
    email = serializers.EmailField(required=True)
    password = serializers.CharField(write_only=True)

    def validate(self, data: dict):

        try:
            email = data['email']
            password = data['password']
        except Exception as e:
            raise Exception(
                {"error_message": "email and password field are required", "status_code": 400, })

        try:
            user = User.objects.get(email=email)
        except Exception as e:
            raise Exception(
                {"error_message": "Invalid email or password.", "status_code": 401,})

        if not user.check_password(password):
            raise Exception(
                {"error_message": "Invalid email or password.", "status_code": 401,})

        return {
            'user': user
        }


class ErrorMessageSerializer(serializers.Serializer):
    error_message = serializers.CharField()
    status_code = serializers.IntegerField()

class LoginResponseSerializer(serializers.Serializer):
    access = serializers.CharField()
    refresh = serializers.CharField()
