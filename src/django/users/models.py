from uuid import uuid4
from django.db import models
from django.utils import timezone
from django.utils.translation import gettext_lazy as _
from django.contrib.auth.models import AbstractBaseUser, PermissionsMixin, BaseUserManager

class UserManager(BaseUserManager):
    def create_user(self, **extra_fields):
        try:
            user: User = self.model(**extra_fields)
            user.set_password(extra_fields.get("password"))
            user.save(using=self._db)
            return user
        except Exception as e:
            print(f"Error while creating user {e}")
            raise ValueError(e)

    def create_superuser(self, email: str = None, password: str = None, phone_number: str = None, **extra_fields):
        extra_fields.setdefault("is_staff", True)
        extra_fields.setdefault("is_superuser", True)
        extra_fields.setdefault("is_active", True)
        return self.create_user(email=email, password=password, phone_number=phone_number,**extra_fields)


class User(AbstractBaseUser, PermissionsMixin):
    GENDER_CHOICES = (
        ("male", "Male"),
        ("female", "Female"),
        ("other", "Other"),
    )


    ROLE_CHOICES = (
        ("project_manager", "ProjectManager"),
        ("developer", "Developer"),
    )
    class Meta:
        default_related_name = _("users")
        indexes = (
            models.Index(fields=("id", "email")),
        )
        ordering = ("-created_at",)
        verbose_name = _("user")
        verbose_name_plural = _("users")

    id = models.UUIDField(_("user id"), default=uuid4,
                          editable=False, primary_key=True)
    email = models.EmailField(_("email"),unique=True)
    phone_number = models.CharField(
        _("phone number"), max_length=10,null=True)
    first_name = models.CharField(_("first name"), max_length=50,)
    last_name = models.CharField(_("last name"), max_length=50,)
    profile_picture = models.URLField(_("profile picture"), null=True, blank=True)
    date_of_birth = models.DateField(_("date of birth"), null=True, blank=True)
    gender = models.CharField(_("gender"), max_length=10, choices=GENDER_CHOICES, null=True)
    role = models.CharField(max_length=20, choices=ROLE_CHOICES, null=True)
    is_active = models.BooleanField(_("is active"), default=False)
    is_superuser = models.BooleanField(_("is super user"), default=False)
    is_staff = models.BooleanField(_("is staff"), default=False)
    is_deleted = models.BooleanField(_("is deleted"), default=False)
    created_at = models.DateTimeField(
        _("date created"), default=timezone.now, editable=False)
    updated_at = models.DateTimeField(
        _("edited date"), editable=False, default=timezone.now)

    USERNAME_FIELD = "email"
    EMAIL_FIELD = "email"

    objects = UserManager()

    def __str__(self) -> str:
        return f"{self.email}"

