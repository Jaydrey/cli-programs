import json
from django.shortcuts import render
from rest_framework import status
from rest_framework.request import Request
from rest_framework.response import Response

from rest_framework.views import APIView
from rest_framework.viewsets import ModelViewSet
from rest_framework import serializers


from rest_framework.permissions import AllowAny

from rest_framework_simplejwt.tokens import RefreshToken

from rest_framework_simplejwt.views import TokenObtainPairView

# serializers
from .serializers import (
    UserLoginSerializer,
    CreateUserSerializer,
    UserSerializer,
    ErrorMessageSerializer,
    LoginResponseSerializer
)

# models
from .models import User

# swagger
from drf_spectacular.utils import (
    extend_schema,
)


class UserViewSet(ModelViewSet):
    queryset = User.objects.all()
    serializer_class = UserSerializer
    permission_classes = (AllowAny,)


    @extend_schema(exclude=True)
    def create(self, request, *args, **kwargs):
        return None


class RegistrationAPIView(APIView):
    permission_classes = (AllowAny,)
    authentication_classes = ()

    @extend_schema(
        request=CreateUserSerializer,
        responses={
            201: UserSerializer,
            400: ErrorMessageSerializer,
        },
    )
    def post(self, request: Request, *args, **kwargs):
        data = request.data
        serializer = CreateUserSerializer(data=data)

        try:
            serializer.is_valid(raise_exception=True)
        except Exception as e:
            if e.args[0].get("status_code") == 400:
                message = ErrorMessageSerializer(data=e.args[0])
                return Response(message, status=status.HTTP_400_BAD_REQUEST)
            message = ErrorMessageSerializer(data={"error_message": f"something went wrong, {e}", "status_code": 500})
            return Response(message, status=status.HTTP_400_BAD_REQUEST)

        user: User = serializer.save()

        serializer_user = UserSerializer(user)
        return Response(serializer_user.data, status=status.HTTP_201_CREATED)


class LoginAPIView(TokenObtainPairView):
    permission_classes = (AllowAny,)
    serializer_class = UserLoginSerializer

    @extend_schema(
        request=UserLoginSerializer,
        responses={
            400: ErrorMessageSerializer,
            401: ErrorMessageSerializer,
            200: LoginResponseSerializer,
        },
    )
    def post(self, request: Request, *args, **kwargs):
        serializer = self.serializer_class(data=request.data)
        try:
            serializer.is_valid(raise_exception=True)
        except Exception as e:
            if e.args[0].get("status_code") == 401:
                message = ErrorMessageSerializer(data=e.args[0])
                return Response(message.initial_data, status=status.HTTP_401_UNAUTHORIZED)
            if e.args[0].get("status_code") == 400:
                message = ErrorMessageSerializer(data=e.args[0])
                return Response(message.initial_data, status=status.HTTP_400_BAD_REQUEST)
            message = ErrorMessageSerializer(data={"error_message": f"something went wrong, {e}", "status_code": 500})
            return Response(message.initial_data, status=status.HTTP_500_INTERNAL_SERVER_ERROR)


        user = serializer.validated_data.get("user")

        refresh = RefreshToken.for_user(user)

        refresh["email"] = str(user.email)

        message = LoginResponseSerializer(data={
            'access': str(refresh.access_token),
            'refresh': str(refresh),
        })

        return Response(message.initial_data, status=status.HTTP_200_OK)



