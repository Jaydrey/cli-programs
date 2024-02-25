from rest_framework import serializers
from django.contrib.auth import authenticate

class UserLoginSerializer(serializers.Serializer):
    email = serializers.EmailField(required=False, allow_null=True)
    password = serializers.CharField(write_only=True)

    def validate(self, data: dict):
        email = data.get('email')
        password = data.get('password')

        if not password:
            raise serializers.ValidationError({"error_message": "Provide password for login"}, code="invalid password")

        if not email:
            raise serializers.ValidationError({"error_message": "Provide email for login"}, code="invalid email")

        user = authenticate(request=self.context.get(
            'request'), email=email, password=password)
        if not user:
            raise serializers.ValidationError({"error_message": "Invalid email or password."}, code="invalid credentials")

        return {
            'user': user
        }

