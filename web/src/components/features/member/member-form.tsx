"use client";

import * as React from "react";
import { useMutation, useQuery, useQueryClient } from "@tanstack/react-query";
import { membersApi, rolesApi, usersApi } from "@/lib/api";
import { Button } from "@/components/ui/button";
import { Select } from "@/components/ui/select";
import { MultiSelect } from "@/components/ui/select";
import {
  Dialog,
  DialogContent,
  DialogDescription,
  DialogFooter,
  DialogHeader,
  DialogTitle,
} from "@/components/ui/dialog";
import type { MemberDetail } from "@/types";

interface MemberFormProps {
  projectId: number;
  open: boolean;
  onOpenChange: (open: boolean) => void;
  member?: MemberDetail | null;
}

export function MemberForm({ projectId, open, onOpenChange, member }: MemberFormProps) {
  const queryClient = useQueryClient();
  const isEdit = Boolean(member);
  const [selectedUserId, setSelectedUserId] = React.useState<string>("");
  const [selectedRoleIds, setSelectedRoleIds] = React.useState<string[]>([]);

  React.useEffect(() => {
    if (member) {
      setSelectedUserId(String(member.user.id));
      setSelectedRoleIds(member.roles.map((role) => String(role.id)));
      return;
    }
    setSelectedUserId("");
    setSelectedRoleIds([]);
  }, [member, open]);

  const { data: usersData } = useQuery({
    queryKey: ["users", "members-form"],
    queryFn: () => usersApi.list({ limit: 100 }),
    enabled: open && !isEdit,
  });

  const { data: rolesData } = useQuery({
    queryKey: ["roles", "members-form"],
    queryFn: () => rolesApi.list(),
    enabled: open,
  });

  const createMutation = useMutation({
    mutationFn: () =>
      membersApi.create(projectId, {
        user_id: Number(selectedUserId),
        role_ids: selectedRoleIds.map((id) => Number(id)),
      }),
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: ["members", projectId] });
      onOpenChange(false);
    },
  });

  const updateMutation = useMutation({
    mutationFn: () =>
      membersApi.update(member!.id, {
        role_ids: selectedRoleIds.map((id) => Number(id)),
      }),
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: ["members", projectId] });
      onOpenChange(false);
    },
  });

  const roleOptions =
    rolesData?.roles.map((role) => ({ value: String(role.id), label: role.name })) ?? [];

  const userOptions =
    usersData?.users.map((user) => ({
      value: String(user.id),
      label: `${user.firstname} ${user.lastname} (${user.login})`,
    })) ?? [];

  const isSubmitting = createMutation.isPending || updateMutation.isPending;
  const canSubmit = isEdit
    ? selectedRoleIds.length > 0
    : Boolean(selectedUserId) && selectedRoleIds.length > 0;

  const handleSubmit = () => {
    if (isEdit) {
      updateMutation.mutate();
      return;
    }
    createMutation.mutate();
  };

  return (
    <Dialog open={open} onOpenChange={onOpenChange}>
      <DialogContent>
        <DialogHeader>
          <DialogTitle>{isEdit ? "Edit Member Roles" : "Add Member"}</DialogTitle>
          <DialogDescription>
            {isEdit ? "Update the member's project roles." : "Add a user to this project and assign roles."}
          </DialogDescription>
        </DialogHeader>

        <div className="space-y-4">
          {!isEdit && (
            <div className="space-y-2">
              <p className="text-sm font-medium">User</p>
              <Select
                value={selectedUserId}
                onValueChange={setSelectedUserId}
                options={[{ value: "", label: "Select a user" }, ...userOptions]}
              />
            </div>
          )}

          <div className="space-y-2">
            <p className="text-sm font-medium">Roles</p>
            <MultiSelect
              value={selectedRoleIds}
              onValueChange={setSelectedRoleIds}
              options={roleOptions}
              placeholder="Select one or more roles"
            />
          </div>
        </div>

        <DialogFooter>
          <Button variant="outline" onClick={() => onOpenChange(false)} disabled={isSubmitting}>
            Cancel
          </Button>
          <Button onClick={handleSubmit} disabled={!canSubmit || isSubmitting}>
            {isSubmitting ? "Saving..." : isEdit ? "Update Roles" : "Add Member"}
          </Button>
        </DialogFooter>
      </DialogContent>
    </Dialog>
  );
}
