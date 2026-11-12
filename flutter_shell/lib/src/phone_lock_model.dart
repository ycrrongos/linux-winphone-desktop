// SPDX-License-Identifier: GPL-3.0-only
bool revealLockCredentials(double progress, double velocity) =>
    velocity < -500 || (velocity <= 300 && progress >= .28);
