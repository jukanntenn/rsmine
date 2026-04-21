"use client";

import { useRouter } from "next/navigation";
import { useForm } from "react-hook-form";
import { zodResolver } from "@hookform/resolvers/zod";
import { z } from "zod";
import { useMutation, useQuery, useQueryClient } from "@tanstack/react-query";
import { usersApi } from "@/lib/api";
import { Button } from "@/components/ui/button";
import { Input } from "@/components/ui/input";
import { Label } from "@/components/ui/label";
import { Select } from "@/components/ui/select";

const userFormSchema = z.object({
  login: z.string().min(1, "Login is required"),
  firstname: z.string().min(1, "First name is required"),
  lastname: z.string().min(1, "Last name is required"),
  mail: z.string().email("Invalid email address"),
  password: z.string().optional(),
  admin: z.boolean().optional(),
  status: z.number().optional(),
});

type UserFormValues = z.infer<typeof userFormSchema>;

interface UserFormProps {
  userId?: number;
}

export function UserForm({ userId }: UserFormProps) {
  const router = useRouter();
  const queryClient = useQueryClient();
  const isEdit = typeof userId === "number";

  const { data: userData } = useQuery({
    queryKey: ["user", userId, "form"],
    queryFn: () => usersApi.get(userId!),
    enabled: isEdit,
  });

  const {
    register,
    setValue,
    watch,
    handleSubmit,
    formState: { errors, isSubmitting },
  } = useForm<UserFormValues>({
    resolver: zodResolver(userFormSchema),
    values: userData?.user
      ? {
          login: userData.user.login,
          firstname: userData.user.firstname,
          lastname: userData.user.lastname,
          mail: userData.user.mail,
          password: "",
          admin: userData.user.admin,
          status: userData.user.status,
        }
      : {
          login: "",
          firstname: "",
          lastname: "",
          mail: "",
          password: "",
          admin: false,
          status: 1,
        },
  });

  const createMutation = useMutation({
    mutationFn: (values: UserFormValues) =>
      usersApi.create({
        login: values.login,
        firstname: values.firstname,
        lastname: values.lastname,
        mail: values.mail,
        password: values.password || undefined,
        admin: values.admin ?? false,
        status: values.status,
      }),
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: ["users"] });
      router.push("/users");
    },
  });

  const updateMutation = useMutation({
    mutationFn: (values: UserFormValues) =>
      usersApi.update(userId!, {
        firstname: values.firstname,
        lastname: values.lastname,
        mail: values.mail,
        password: values.password || undefined,
        admin: values.admin,
        status: values.status,
      }),
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: ["users"] });
      queryClient.invalidateQueries({ queryKey: ["user", userId] });
      router.push("/users");
    },
  });

  const onSubmit = (values: UserFormValues) => {
    if (isEdit) {
      updateMutation.mutate(values);
      return;
    }
    createMutation.mutate(values);
  };

  const saving =
    isSubmitting || createMutation.isPending || updateMutation.isPending;

  return (
    <form onSubmit={handleSubmit(onSubmit)} className="space-y-6">
      <div className="grid gap-6 md:grid-cols-2">
        <div className="space-y-2">
          <Label htmlFor="login">Login</Label>
          <Input id="login" {...register("login")} disabled={isEdit} />
          {errors.login && (
            <p className="text-sm text-destructive">{errors.login.message}</p>
          )}
        </div>
        <div className="space-y-2">
          <Label htmlFor="mail">Email</Label>
          <Input id="mail" type="email" {...register("mail")} />
          {errors.mail && (
            <p className="text-sm text-destructive">{errors.mail.message}</p>
          )}
        </div>
      </div>

      <div className="grid gap-6 md:grid-cols-2">
        <div className="space-y-2">
          <Label htmlFor="firstname">First Name</Label>
          <Input id="firstname" {...register("firstname")} />
          {errors.firstname && (
            <p className="text-sm text-destructive">
              {errors.firstname.message}
            </p>
          )}
        </div>
        <div className="space-y-2">
          <Label htmlFor="lastname">Last Name</Label>
          <Input id="lastname" {...register("lastname")} />
          {errors.lastname && (
            <p className="text-sm text-destructive">
              {errors.lastname.message}
            </p>
          )}
        </div>
      </div>

      <div className="space-y-2">
        <Label htmlFor="password">{isEdit ? "New Password" : "Password"}</Label>
        <Input id="password" type="password" {...register("password")} />
      </div>

      <div className="grid gap-6 md:grid-cols-2">
        <div className="space-y-2">
          <Label htmlFor="status">Status</Label>
          <Select
            id="status"
            value={String(watch("status") ?? 1)}
            onValueChange={(value) =>
              setValue("status", Number(value), { shouldDirty: true })
            }
            options={[
              { value: "1", label: "Active" },
              { value: "2", label: "Registered" },
              { value: "3", label: "Locked" },
            ]}
          />
        </div>
        <div className="space-y-2">
          <Label htmlFor="admin">Admin</Label>
          <Select
            id="admin"
            value={String(Boolean(watch("admin")))}
            onValueChange={(value) =>
              setValue("admin", value === "true", { shouldDirty: true })
            }
            options={[
              { value: "false", label: "No" },
              { value: "true", label: "Yes" },
            ]}
          />
        </div>
      </div>

      <div className="flex items-center gap-3">
        <Button type="submit" disabled={saving}>
          {saving ? "Saving..." : isEdit ? "Update User" : "Create User"}
        </Button>
        <Button
          type="button"
          variant="outline"
          onClick={() => router.back()}
          disabled={saving}
        >
          Cancel
        </Button>
      </div>
    </form>
  );
}
