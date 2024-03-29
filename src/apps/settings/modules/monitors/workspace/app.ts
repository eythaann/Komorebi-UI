import { GeneralSettingsSelectors, getWorkspaceSelector } from '../../shared/app/selectors';
import { defaultOnNull } from '../../shared/app/utils';

import { RootState } from '../../shared/domain/state';

export const getWorkspacePaddingSelector = (idx: number, monitorIdx: number) => (state: RootState) => {
  return defaultOnNull(
    getWorkspaceSelector(idx, monitorIdx)(state)?.workspacePadding,
    GeneralSettingsSelectors.workspacePadding(state),
  );
};

export const getContainerPaddingSelector = (idx: number, monitorIdx: number) => (state: RootState) => {
  return defaultOnNull(
    getWorkspaceSelector(idx, monitorIdx)(state)?.containerPadding,
    GeneralSettingsSelectors.containerPadding(state),
  );
};
