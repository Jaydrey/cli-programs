from django.shortcuts import render
from rest_framework import status
from rest_framework.request import Request
from rest_framework.response import Response

from rest_framework.views import APIView
from rest_framework import serializers


from rest_framework.permissions import AllowAny

from rest_framework_simplejwt.tokens import RefreshToken

from rest_framework_simplejwt.views import TokenObtainPairView

from .serializers import (
    UserSerializer,
    UserLoginSerializer,
)
# swagger
from drf_spectacular.utils import (
    extend_schema,
)

class LoginAPIView(TokenObtainPairView):
    permission_classes = (AllowAny,)
    serializer_class = UserLoginSerializer

    @extend_schema(
        request=UserLoginSerializer,
        responses={
            400: str,
            401: str,
            200: dict[str, str],
        },
    )
    def post(self, request: Request, *args, **kwargs):
        serializer = self.serializer_class(data=request.data)
        try:
            serializer.is_valid(raise_exception=True)
        except serializers.ValidationError as e:
            if "invalid_password" in e.detail:
                return Response({"errors": e.detail}, status=status.HTTP_400_BAD_REQUEST)
            if "invalid_email" in e.detail:
                return Response({"errors": e.detail}, status=status.HTTP_400_BAD_REQUEST)
            if "invalid_credentials" in e.detail:
                return Response({"errors": e.detail}, status=status.HTTP_401_UNAUTHORIZED)

            return Response({"errors": e.detail}, status=status.HTTP_400_BAD_REQUEST)


        user = serializer.validated_data.get("user")

        refresh = RefreshToken.for_user(user)

        refresh["email"] = str(user.email)

        return Response({
            'access': str(refresh.access_token),
            'refresh': str(refresh),
        }, status=status.HTTP_200_OK)



