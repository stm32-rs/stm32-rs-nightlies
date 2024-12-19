///Register `TSC_IOSCR` reader
pub type R = crate::R<TSC_IOSCRrs>;
///Register `TSC_IOSCR` writer
pub type W = crate::W<TSC_IOSCRrs>;
///Field `G1_IO1` reader - Gx_IOy sampling mode These bits are set and cleared by software to configure the Gx_IOy as a sampling capacitor I/O. Only one I/O per analog I/O group must be defined as sampling capacitor. Note: These bits must not be modified when an acquisition is ongoing. Note: During the acquisition phase and even if the TSC peripheral alternate function is not enabled, as soon as the TSC_IOSCR bit is set, the corresponding GPIO analog switch is automatically controlled by the touch sensing controller.
pub type G1_IO1_R = crate::BitReader;
///Field `G1_IO1` writer - Gx_IOy sampling mode These bits are set and cleared by software to configure the Gx_IOy as a sampling capacitor I/O. Only one I/O per analog I/O group must be defined as sampling capacitor. Note: These bits must not be modified when an acquisition is ongoing. Note: During the acquisition phase and even if the TSC peripheral alternate function is not enabled, as soon as the TSC_IOSCR bit is set, the corresponding GPIO analog switch is automatically controlled by the touch sensing controller.
pub type G1_IO1_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `G1_IO2` reader - Gx_IOy sampling mode These bits are set and cleared by software to configure the Gx_IOy as a sampling capacitor I/O. Only one I/O per analog I/O group must be defined as sampling capacitor. Note: These bits must not be modified when an acquisition is ongoing. Note: During the acquisition phase and even if the TSC peripheral alternate function is not enabled, as soon as the TSC_IOSCR bit is set, the corresponding GPIO analog switch is automatically controlled by the touch sensing controller.
pub type G1_IO2_R = crate::BitReader;
///Field `G1_IO2` writer - Gx_IOy sampling mode These bits are set and cleared by software to configure the Gx_IOy as a sampling capacitor I/O. Only one I/O per analog I/O group must be defined as sampling capacitor. Note: These bits must not be modified when an acquisition is ongoing. Note: During the acquisition phase and even if the TSC peripheral alternate function is not enabled, as soon as the TSC_IOSCR bit is set, the corresponding GPIO analog switch is automatically controlled by the touch sensing controller.
pub type G1_IO2_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `G1_IO3` reader - Gx_IOy sampling mode These bits are set and cleared by software to configure the Gx_IOy as a sampling capacitor I/O. Only one I/O per analog I/O group must be defined as sampling capacitor. Note: These bits must not be modified when an acquisition is ongoing. Note: During the acquisition phase and even if the TSC peripheral alternate function is not enabled, as soon as the TSC_IOSCR bit is set, the corresponding GPIO analog switch is automatically controlled by the touch sensing controller.
pub type G1_IO3_R = crate::BitReader;
///Field `G1_IO3` writer - Gx_IOy sampling mode These bits are set and cleared by software to configure the Gx_IOy as a sampling capacitor I/O. Only one I/O per analog I/O group must be defined as sampling capacitor. Note: These bits must not be modified when an acquisition is ongoing. Note: During the acquisition phase and even if the TSC peripheral alternate function is not enabled, as soon as the TSC_IOSCR bit is set, the corresponding GPIO analog switch is automatically controlled by the touch sensing controller.
pub type G1_IO3_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `G1_IO4` reader - Gx_IOy sampling mode These bits are set and cleared by software to configure the Gx_IOy as a sampling capacitor I/O. Only one I/O per analog I/O group must be defined as sampling capacitor. Note: These bits must not be modified when an acquisition is ongoing. Note: During the acquisition phase and even if the TSC peripheral alternate function is not enabled, as soon as the TSC_IOSCR bit is set, the corresponding GPIO analog switch is automatically controlled by the touch sensing controller.
pub type G1_IO4_R = crate::BitReader;
///Field `G1_IO4` writer - Gx_IOy sampling mode These bits are set and cleared by software to configure the Gx_IOy as a sampling capacitor I/O. Only one I/O per analog I/O group must be defined as sampling capacitor. Note: These bits must not be modified when an acquisition is ongoing. Note: During the acquisition phase and even if the TSC peripheral alternate function is not enabled, as soon as the TSC_IOSCR bit is set, the corresponding GPIO analog switch is automatically controlled by the touch sensing controller.
pub type G1_IO4_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `G2_IO1` reader - Gx_IOy sampling mode These bits are set and cleared by software to configure the Gx_IOy as a sampling capacitor I/O. Only one I/O per analog I/O group must be defined as sampling capacitor. Note: These bits must not be modified when an acquisition is ongoing. Note: During the acquisition phase and even if the TSC peripheral alternate function is not enabled, as soon as the TSC_IOSCR bit is set, the corresponding GPIO analog switch is automatically controlled by the touch sensing controller.
pub type G2_IO1_R = crate::BitReader;
///Field `G2_IO1` writer - Gx_IOy sampling mode These bits are set and cleared by software to configure the Gx_IOy as a sampling capacitor I/O. Only one I/O per analog I/O group must be defined as sampling capacitor. Note: These bits must not be modified when an acquisition is ongoing. Note: During the acquisition phase and even if the TSC peripheral alternate function is not enabled, as soon as the TSC_IOSCR bit is set, the corresponding GPIO analog switch is automatically controlled by the touch sensing controller.
pub type G2_IO1_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `G2_IO2` reader - Gx_IOy sampling mode These bits are set and cleared by software to configure the Gx_IOy as a sampling capacitor I/O. Only one I/O per analog I/O group must be defined as sampling capacitor. Note: These bits must not be modified when an acquisition is ongoing. Note: During the acquisition phase and even if the TSC peripheral alternate function is not enabled, as soon as the TSC_IOSCR bit is set, the corresponding GPIO analog switch is automatically controlled by the touch sensing controller.
pub type G2_IO2_R = crate::BitReader;
///Field `G2_IO2` writer - Gx_IOy sampling mode These bits are set and cleared by software to configure the Gx_IOy as a sampling capacitor I/O. Only one I/O per analog I/O group must be defined as sampling capacitor. Note: These bits must not be modified when an acquisition is ongoing. Note: During the acquisition phase and even if the TSC peripheral alternate function is not enabled, as soon as the TSC_IOSCR bit is set, the corresponding GPIO analog switch is automatically controlled by the touch sensing controller.
pub type G2_IO2_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `G2_IO3` reader - Gx_IOy sampling mode These bits are set and cleared by software to configure the Gx_IOy as a sampling capacitor I/O. Only one I/O per analog I/O group must be defined as sampling capacitor. Note: These bits must not be modified when an acquisition is ongoing. Note: During the acquisition phase and even if the TSC peripheral alternate function is not enabled, as soon as the TSC_IOSCR bit is set, the corresponding GPIO analog switch is automatically controlled by the touch sensing controller.
pub type G2_IO3_R = crate::BitReader;
///Field `G2_IO3` writer - Gx_IOy sampling mode These bits are set and cleared by software to configure the Gx_IOy as a sampling capacitor I/O. Only one I/O per analog I/O group must be defined as sampling capacitor. Note: These bits must not be modified when an acquisition is ongoing. Note: During the acquisition phase and even if the TSC peripheral alternate function is not enabled, as soon as the TSC_IOSCR bit is set, the corresponding GPIO analog switch is automatically controlled by the touch sensing controller.
pub type G2_IO3_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `G2_IO4` reader - Gx_IOy sampling mode These bits are set and cleared by software to configure the Gx_IOy as a sampling capacitor I/O. Only one I/O per analog I/O group must be defined as sampling capacitor. Note: These bits must not be modified when an acquisition is ongoing. Note: During the acquisition phase and even if the TSC peripheral alternate function is not enabled, as soon as the TSC_IOSCR bit is set, the corresponding GPIO analog switch is automatically controlled by the touch sensing controller.
pub type G2_IO4_R = crate::BitReader;
///Field `G2_IO4` writer - Gx_IOy sampling mode These bits are set and cleared by software to configure the Gx_IOy as a sampling capacitor I/O. Only one I/O per analog I/O group must be defined as sampling capacitor. Note: These bits must not be modified when an acquisition is ongoing. Note: During the acquisition phase and even if the TSC peripheral alternate function is not enabled, as soon as the TSC_IOSCR bit is set, the corresponding GPIO analog switch is automatically controlled by the touch sensing controller.
pub type G2_IO4_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `G3_IO1` reader - Gx_IOy sampling mode These bits are set and cleared by software to configure the Gx_IOy as a sampling capacitor I/O. Only one I/O per analog I/O group must be defined as sampling capacitor. Note: These bits must not be modified when an acquisition is ongoing. Note: During the acquisition phase and even if the TSC peripheral alternate function is not enabled, as soon as the TSC_IOSCR bit is set, the corresponding GPIO analog switch is automatically controlled by the touch sensing controller.
pub type G3_IO1_R = crate::BitReader;
///Field `G3_IO1` writer - Gx_IOy sampling mode These bits are set and cleared by software to configure the Gx_IOy as a sampling capacitor I/O. Only one I/O per analog I/O group must be defined as sampling capacitor. Note: These bits must not be modified when an acquisition is ongoing. Note: During the acquisition phase and even if the TSC peripheral alternate function is not enabled, as soon as the TSC_IOSCR bit is set, the corresponding GPIO analog switch is automatically controlled by the touch sensing controller.
pub type G3_IO1_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `G3_IO2` reader - Gx_IOy sampling mode These bits are set and cleared by software to configure the Gx_IOy as a sampling capacitor I/O. Only one I/O per analog I/O group must be defined as sampling capacitor. Note: These bits must not be modified when an acquisition is ongoing. Note: During the acquisition phase and even if the TSC peripheral alternate function is not enabled, as soon as the TSC_IOSCR bit is set, the corresponding GPIO analog switch is automatically controlled by the touch sensing controller.
pub type G3_IO2_R = crate::BitReader;
///Field `G3_IO2` writer - Gx_IOy sampling mode These bits are set and cleared by software to configure the Gx_IOy as a sampling capacitor I/O. Only one I/O per analog I/O group must be defined as sampling capacitor. Note: These bits must not be modified when an acquisition is ongoing. Note: During the acquisition phase and even if the TSC peripheral alternate function is not enabled, as soon as the TSC_IOSCR bit is set, the corresponding GPIO analog switch is automatically controlled by the touch sensing controller.
pub type G3_IO2_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `G3_IO3` reader - Gx_IOy sampling mode These bits are set and cleared by software to configure the Gx_IOy as a sampling capacitor I/O. Only one I/O per analog I/O group must be defined as sampling capacitor. Note: These bits must not be modified when an acquisition is ongoing. Note: During the acquisition phase and even if the TSC peripheral alternate function is not enabled, as soon as the TSC_IOSCR bit is set, the corresponding GPIO analog switch is automatically controlled by the touch sensing controller.
pub type G3_IO3_R = crate::BitReader;
///Field `G3_IO3` writer - Gx_IOy sampling mode These bits are set and cleared by software to configure the Gx_IOy as a sampling capacitor I/O. Only one I/O per analog I/O group must be defined as sampling capacitor. Note: These bits must not be modified when an acquisition is ongoing. Note: During the acquisition phase and even if the TSC peripheral alternate function is not enabled, as soon as the TSC_IOSCR bit is set, the corresponding GPIO analog switch is automatically controlled by the touch sensing controller.
pub type G3_IO3_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `G3_IO4` reader - Gx_IOy sampling mode These bits are set and cleared by software to configure the Gx_IOy as a sampling capacitor I/O. Only one I/O per analog I/O group must be defined as sampling capacitor. Note: These bits must not be modified when an acquisition is ongoing. Note: During the acquisition phase and even if the TSC peripheral alternate function is not enabled, as soon as the TSC_IOSCR bit is set, the corresponding GPIO analog switch is automatically controlled by the touch sensing controller.
pub type G3_IO4_R = crate::BitReader;
///Field `G3_IO4` writer - Gx_IOy sampling mode These bits are set and cleared by software to configure the Gx_IOy as a sampling capacitor I/O. Only one I/O per analog I/O group must be defined as sampling capacitor. Note: These bits must not be modified when an acquisition is ongoing. Note: During the acquisition phase and even if the TSC peripheral alternate function is not enabled, as soon as the TSC_IOSCR bit is set, the corresponding GPIO analog switch is automatically controlled by the touch sensing controller.
pub type G3_IO4_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `G4_IO1` reader - Gx_IOy sampling mode These bits are set and cleared by software to configure the Gx_IOy as a sampling capacitor I/O. Only one I/O per analog I/O group must be defined as sampling capacitor. Note: These bits must not be modified when an acquisition is ongoing. Note: During the acquisition phase and even if the TSC peripheral alternate function is not enabled, as soon as the TSC_IOSCR bit is set, the corresponding GPIO analog switch is automatically controlled by the touch sensing controller.
pub type G4_IO1_R = crate::BitReader;
///Field `G4_IO1` writer - Gx_IOy sampling mode These bits are set and cleared by software to configure the Gx_IOy as a sampling capacitor I/O. Only one I/O per analog I/O group must be defined as sampling capacitor. Note: These bits must not be modified when an acquisition is ongoing. Note: During the acquisition phase and even if the TSC peripheral alternate function is not enabled, as soon as the TSC_IOSCR bit is set, the corresponding GPIO analog switch is automatically controlled by the touch sensing controller.
pub type G4_IO1_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `G4_IO2` reader - Gx_IOy sampling mode These bits are set and cleared by software to configure the Gx_IOy as a sampling capacitor I/O. Only one I/O per analog I/O group must be defined as sampling capacitor. Note: These bits must not be modified when an acquisition is ongoing. Note: During the acquisition phase and even if the TSC peripheral alternate function is not enabled, as soon as the TSC_IOSCR bit is set, the corresponding GPIO analog switch is automatically controlled by the touch sensing controller.
pub type G4_IO2_R = crate::BitReader;
///Field `G4_IO2` writer - Gx_IOy sampling mode These bits are set and cleared by software to configure the Gx_IOy as a sampling capacitor I/O. Only one I/O per analog I/O group must be defined as sampling capacitor. Note: These bits must not be modified when an acquisition is ongoing. Note: During the acquisition phase and even if the TSC peripheral alternate function is not enabled, as soon as the TSC_IOSCR bit is set, the corresponding GPIO analog switch is automatically controlled by the touch sensing controller.
pub type G4_IO2_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `G4_IO3` reader - Gx_IOy sampling mode These bits are set and cleared by software to configure the Gx_IOy as a sampling capacitor I/O. Only one I/O per analog I/O group must be defined as sampling capacitor. Note: These bits must not be modified when an acquisition is ongoing. Note: During the acquisition phase and even if the TSC peripheral alternate function is not enabled, as soon as the TSC_IOSCR bit is set, the corresponding GPIO analog switch is automatically controlled by the touch sensing controller.
pub type G4_IO3_R = crate::BitReader;
///Field `G4_IO3` writer - Gx_IOy sampling mode These bits are set and cleared by software to configure the Gx_IOy as a sampling capacitor I/O. Only one I/O per analog I/O group must be defined as sampling capacitor. Note: These bits must not be modified when an acquisition is ongoing. Note: During the acquisition phase and even if the TSC peripheral alternate function is not enabled, as soon as the TSC_IOSCR bit is set, the corresponding GPIO analog switch is automatically controlled by the touch sensing controller.
pub type G4_IO3_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `G4_IO4` reader - Gx_IOy sampling mode These bits are set and cleared by software to configure the Gx_IOy as a sampling capacitor I/O. Only one I/O per analog I/O group must be defined as sampling capacitor. Note: These bits must not be modified when an acquisition is ongoing. Note: During the acquisition phase and even if the TSC peripheral alternate function is not enabled, as soon as the TSC_IOSCR bit is set, the corresponding GPIO analog switch is automatically controlled by the touch sensing controller.
pub type G4_IO4_R = crate::BitReader;
///Field `G4_IO4` writer - Gx_IOy sampling mode These bits are set and cleared by software to configure the Gx_IOy as a sampling capacitor I/O. Only one I/O per analog I/O group must be defined as sampling capacitor. Note: These bits must not be modified when an acquisition is ongoing. Note: During the acquisition phase and even if the TSC peripheral alternate function is not enabled, as soon as the TSC_IOSCR bit is set, the corresponding GPIO analog switch is automatically controlled by the touch sensing controller.
pub type G4_IO4_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `G5_IO1` reader - Gx_IOy sampling mode These bits are set and cleared by software to configure the Gx_IOy as a sampling capacitor I/O. Only one I/O per analog I/O group must be defined as sampling capacitor. Note: These bits must not be modified when an acquisition is ongoing. Note: During the acquisition phase and even if the TSC peripheral alternate function is not enabled, as soon as the TSC_IOSCR bit is set, the corresponding GPIO analog switch is automatically controlled by the touch sensing controller.
pub type G5_IO1_R = crate::BitReader;
///Field `G5_IO1` writer - Gx_IOy sampling mode These bits are set and cleared by software to configure the Gx_IOy as a sampling capacitor I/O. Only one I/O per analog I/O group must be defined as sampling capacitor. Note: These bits must not be modified when an acquisition is ongoing. Note: During the acquisition phase and even if the TSC peripheral alternate function is not enabled, as soon as the TSC_IOSCR bit is set, the corresponding GPIO analog switch is automatically controlled by the touch sensing controller.
pub type G5_IO1_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `G5_IO2` reader - Gx_IOy sampling mode These bits are set and cleared by software to configure the Gx_IOy as a sampling capacitor I/O. Only one I/O per analog I/O group must be defined as sampling capacitor. Note: These bits must not be modified when an acquisition is ongoing. Note: During the acquisition phase and even if the TSC peripheral alternate function is not enabled, as soon as the TSC_IOSCR bit is set, the corresponding GPIO analog switch is automatically controlled by the touch sensing controller.
pub type G5_IO2_R = crate::BitReader;
///Field `G5_IO2` writer - Gx_IOy sampling mode These bits are set and cleared by software to configure the Gx_IOy as a sampling capacitor I/O. Only one I/O per analog I/O group must be defined as sampling capacitor. Note: These bits must not be modified when an acquisition is ongoing. Note: During the acquisition phase and even if the TSC peripheral alternate function is not enabled, as soon as the TSC_IOSCR bit is set, the corresponding GPIO analog switch is automatically controlled by the touch sensing controller.
pub type G5_IO2_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `G5_IO3` reader - Gx_IOy sampling mode These bits are set and cleared by software to configure the Gx_IOy as a sampling capacitor I/O. Only one I/O per analog I/O group must be defined as sampling capacitor. Note: These bits must not be modified when an acquisition is ongoing. Note: During the acquisition phase and even if the TSC peripheral alternate function is not enabled, as soon as the TSC_IOSCR bit is set, the corresponding GPIO analog switch is automatically controlled by the touch sensing controller.
pub type G5_IO3_R = crate::BitReader;
///Field `G5_IO3` writer - Gx_IOy sampling mode These bits are set and cleared by software to configure the Gx_IOy as a sampling capacitor I/O. Only one I/O per analog I/O group must be defined as sampling capacitor. Note: These bits must not be modified when an acquisition is ongoing. Note: During the acquisition phase and even if the TSC peripheral alternate function is not enabled, as soon as the TSC_IOSCR bit is set, the corresponding GPIO analog switch is automatically controlled by the touch sensing controller.
pub type G5_IO3_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `G5_IO4` reader - Gx_IOy sampling mode These bits are set and cleared by software to configure the Gx_IOy as a sampling capacitor I/O. Only one I/O per analog I/O group must be defined as sampling capacitor. Note: These bits must not be modified when an acquisition is ongoing. Note: During the acquisition phase and even if the TSC peripheral alternate function is not enabled, as soon as the TSC_IOSCR bit is set, the corresponding GPIO analog switch is automatically controlled by the touch sensing controller.
pub type G5_IO4_R = crate::BitReader;
///Field `G5_IO4` writer - Gx_IOy sampling mode These bits are set and cleared by software to configure the Gx_IOy as a sampling capacitor I/O. Only one I/O per analog I/O group must be defined as sampling capacitor. Note: These bits must not be modified when an acquisition is ongoing. Note: During the acquisition phase and even if the TSC peripheral alternate function is not enabled, as soon as the TSC_IOSCR bit is set, the corresponding GPIO analog switch is automatically controlled by the touch sensing controller.
pub type G5_IO4_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `G6_IO1` reader - Gx_IOy sampling mode These bits are set and cleared by software to configure the Gx_IOy as a sampling capacitor I/O. Only one I/O per analog I/O group must be defined as sampling capacitor. Note: These bits must not be modified when an acquisition is ongoing. Note: During the acquisition phase and even if the TSC peripheral alternate function is not enabled, as soon as the TSC_IOSCR bit is set, the corresponding GPIO analog switch is automatically controlled by the touch sensing controller.
pub type G6_IO1_R = crate::BitReader;
///Field `G6_IO1` writer - Gx_IOy sampling mode These bits are set and cleared by software to configure the Gx_IOy as a sampling capacitor I/O. Only one I/O per analog I/O group must be defined as sampling capacitor. Note: These bits must not be modified when an acquisition is ongoing. Note: During the acquisition phase and even if the TSC peripheral alternate function is not enabled, as soon as the TSC_IOSCR bit is set, the corresponding GPIO analog switch is automatically controlled by the touch sensing controller.
pub type G6_IO1_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `G6_IO2` reader - Gx_IOy sampling mode These bits are set and cleared by software to configure the Gx_IOy as a sampling capacitor I/O. Only one I/O per analog I/O group must be defined as sampling capacitor. Note: These bits must not be modified when an acquisition is ongoing. Note: During the acquisition phase and even if the TSC peripheral alternate function is not enabled, as soon as the TSC_IOSCR bit is set, the corresponding GPIO analog switch is automatically controlled by the touch sensing controller.
pub type G6_IO2_R = crate::BitReader;
///Field `G6_IO2` writer - Gx_IOy sampling mode These bits are set and cleared by software to configure the Gx_IOy as a sampling capacitor I/O. Only one I/O per analog I/O group must be defined as sampling capacitor. Note: These bits must not be modified when an acquisition is ongoing. Note: During the acquisition phase and even if the TSC peripheral alternate function is not enabled, as soon as the TSC_IOSCR bit is set, the corresponding GPIO analog switch is automatically controlled by the touch sensing controller.
pub type G6_IO2_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `G6_IO3` reader - Gx_IOy sampling mode These bits are set and cleared by software to configure the Gx_IOy as a sampling capacitor I/O. Only one I/O per analog I/O group must be defined as sampling capacitor. Note: These bits must not be modified when an acquisition is ongoing. Note: During the acquisition phase and even if the TSC peripheral alternate function is not enabled, as soon as the TSC_IOSCR bit is set, the corresponding GPIO analog switch is automatically controlled by the touch sensing controller.
pub type G6_IO3_R = crate::BitReader;
///Field `G6_IO3` writer - Gx_IOy sampling mode These bits are set and cleared by software to configure the Gx_IOy as a sampling capacitor I/O. Only one I/O per analog I/O group must be defined as sampling capacitor. Note: These bits must not be modified when an acquisition is ongoing. Note: During the acquisition phase and even if the TSC peripheral alternate function is not enabled, as soon as the TSC_IOSCR bit is set, the corresponding GPIO analog switch is automatically controlled by the touch sensing controller.
pub type G6_IO3_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `G6_IO4` reader - Gx_IOy sampling mode These bits are set and cleared by software to configure the Gx_IOy as a sampling capacitor I/O. Only one I/O per analog I/O group must be defined as sampling capacitor. Note: These bits must not be modified when an acquisition is ongoing. Note: During the acquisition phase and even if the TSC peripheral alternate function is not enabled, as soon as the TSC_IOSCR bit is set, the corresponding GPIO analog switch is automatically controlled by the touch sensing controller.
pub type G6_IO4_R = crate::BitReader;
///Field `G6_IO4` writer - Gx_IOy sampling mode These bits are set and cleared by software to configure the Gx_IOy as a sampling capacitor I/O. Only one I/O per analog I/O group must be defined as sampling capacitor. Note: These bits must not be modified when an acquisition is ongoing. Note: During the acquisition phase and even if the TSC peripheral alternate function is not enabled, as soon as the TSC_IOSCR bit is set, the corresponding GPIO analog switch is automatically controlled by the touch sensing controller.
pub type G6_IO4_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `G7_IO1` reader - Gx_IOy sampling mode These bits are set and cleared by software to configure the Gx_IOy as a sampling capacitor I/O. Only one I/O per analog I/O group must be defined as sampling capacitor. Note: These bits must not be modified when an acquisition is ongoing. Note: During the acquisition phase and even if the TSC peripheral alternate function is not enabled, as soon as the TSC_IOSCR bit is set, the corresponding GPIO analog switch is automatically controlled by the touch sensing controller.
pub type G7_IO1_R = crate::BitReader;
///Field `G7_IO1` writer - Gx_IOy sampling mode These bits are set and cleared by software to configure the Gx_IOy as a sampling capacitor I/O. Only one I/O per analog I/O group must be defined as sampling capacitor. Note: These bits must not be modified when an acquisition is ongoing. Note: During the acquisition phase and even if the TSC peripheral alternate function is not enabled, as soon as the TSC_IOSCR bit is set, the corresponding GPIO analog switch is automatically controlled by the touch sensing controller.
pub type G7_IO1_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `G7_IO2` reader - Gx_IOy sampling mode These bits are set and cleared by software to configure the Gx_IOy as a sampling capacitor I/O. Only one I/O per analog I/O group must be defined as sampling capacitor. Note: These bits must not be modified when an acquisition is ongoing. Note: During the acquisition phase and even if the TSC peripheral alternate function is not enabled, as soon as the TSC_IOSCR bit is set, the corresponding GPIO analog switch is automatically controlled by the touch sensing controller.
pub type G7_IO2_R = crate::BitReader;
///Field `G7_IO2` writer - Gx_IOy sampling mode These bits are set and cleared by software to configure the Gx_IOy as a sampling capacitor I/O. Only one I/O per analog I/O group must be defined as sampling capacitor. Note: These bits must not be modified when an acquisition is ongoing. Note: During the acquisition phase and even if the TSC peripheral alternate function is not enabled, as soon as the TSC_IOSCR bit is set, the corresponding GPIO analog switch is automatically controlled by the touch sensing controller.
pub type G7_IO2_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `G7_IO3` reader - Gx_IOy sampling mode These bits are set and cleared by software to configure the Gx_IOy as a sampling capacitor I/O. Only one I/O per analog I/O group must be defined as sampling capacitor. Note: These bits must not be modified when an acquisition is ongoing. Note: During the acquisition phase and even if the TSC peripheral alternate function is not enabled, as soon as the TSC_IOSCR bit is set, the corresponding GPIO analog switch is automatically controlled by the touch sensing controller.
pub type G7_IO3_R = crate::BitReader;
///Field `G7_IO3` writer - Gx_IOy sampling mode These bits are set and cleared by software to configure the Gx_IOy as a sampling capacitor I/O. Only one I/O per analog I/O group must be defined as sampling capacitor. Note: These bits must not be modified when an acquisition is ongoing. Note: During the acquisition phase and even if the TSC peripheral alternate function is not enabled, as soon as the TSC_IOSCR bit is set, the corresponding GPIO analog switch is automatically controlled by the touch sensing controller.
pub type G7_IO3_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `G7_IO4` reader - Gx_IOy sampling mode These bits are set and cleared by software to configure the Gx_IOy as a sampling capacitor I/O. Only one I/O per analog I/O group must be defined as sampling capacitor. Note: These bits must not be modified when an acquisition is ongoing. Note: During the acquisition phase and even if the TSC peripheral alternate function is not enabled, as soon as the TSC_IOSCR bit is set, the corresponding GPIO analog switch is automatically controlled by the touch sensing controller.
pub type G7_IO4_R = crate::BitReader;
///Field `G7_IO4` writer - Gx_IOy sampling mode These bits are set and cleared by software to configure the Gx_IOy as a sampling capacitor I/O. Only one I/O per analog I/O group must be defined as sampling capacitor. Note: These bits must not be modified when an acquisition is ongoing. Note: During the acquisition phase and even if the TSC peripheral alternate function is not enabled, as soon as the TSC_IOSCR bit is set, the corresponding GPIO analog switch is automatically controlled by the touch sensing controller.
pub type G7_IO4_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - Gx_IOy sampling mode These bits are set and cleared by software to configure the Gx_IOy as a sampling capacitor I/O. Only one I/O per analog I/O group must be defined as sampling capacitor. Note: These bits must not be modified when an acquisition is ongoing. Note: During the acquisition phase and even if the TSC peripheral alternate function is not enabled, as soon as the TSC_IOSCR bit is set, the corresponding GPIO analog switch is automatically controlled by the touch sensing controller.
    #[inline(always)]
    pub fn g1_io1(&self) -> G1_IO1_R {
        G1_IO1_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Gx_IOy sampling mode These bits are set and cleared by software to configure the Gx_IOy as a sampling capacitor I/O. Only one I/O per analog I/O group must be defined as sampling capacitor. Note: These bits must not be modified when an acquisition is ongoing. Note: During the acquisition phase and even if the TSC peripheral alternate function is not enabled, as soon as the TSC_IOSCR bit is set, the corresponding GPIO analog switch is automatically controlled by the touch sensing controller.
    #[inline(always)]
    pub fn g1_io2(&self) -> G1_IO2_R {
        G1_IO2_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Gx_IOy sampling mode These bits are set and cleared by software to configure the Gx_IOy as a sampling capacitor I/O. Only one I/O per analog I/O group must be defined as sampling capacitor. Note: These bits must not be modified when an acquisition is ongoing. Note: During the acquisition phase and even if the TSC peripheral alternate function is not enabled, as soon as the TSC_IOSCR bit is set, the corresponding GPIO analog switch is automatically controlled by the touch sensing controller.
    #[inline(always)]
    pub fn g1_io3(&self) -> G1_IO3_R {
        G1_IO3_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Gx_IOy sampling mode These bits are set and cleared by software to configure the Gx_IOy as a sampling capacitor I/O. Only one I/O per analog I/O group must be defined as sampling capacitor. Note: These bits must not be modified when an acquisition is ongoing. Note: During the acquisition phase and even if the TSC peripheral alternate function is not enabled, as soon as the TSC_IOSCR bit is set, the corresponding GPIO analog switch is automatically controlled by the touch sensing controller.
    #[inline(always)]
    pub fn g1_io4(&self) -> G1_IO4_R {
        G1_IO4_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Gx_IOy sampling mode These bits are set and cleared by software to configure the Gx_IOy as a sampling capacitor I/O. Only one I/O per analog I/O group must be defined as sampling capacitor. Note: These bits must not be modified when an acquisition is ongoing. Note: During the acquisition phase and even if the TSC peripheral alternate function is not enabled, as soon as the TSC_IOSCR bit is set, the corresponding GPIO analog switch is automatically controlled by the touch sensing controller.
    #[inline(always)]
    pub fn g2_io1(&self) -> G2_IO1_R {
        G2_IO1_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Gx_IOy sampling mode These bits are set and cleared by software to configure the Gx_IOy as a sampling capacitor I/O. Only one I/O per analog I/O group must be defined as sampling capacitor. Note: These bits must not be modified when an acquisition is ongoing. Note: During the acquisition phase and even if the TSC peripheral alternate function is not enabled, as soon as the TSC_IOSCR bit is set, the corresponding GPIO analog switch is automatically controlled by the touch sensing controller.
    #[inline(always)]
    pub fn g2_io2(&self) -> G2_IO2_R {
        G2_IO2_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - Gx_IOy sampling mode These bits are set and cleared by software to configure the Gx_IOy as a sampling capacitor I/O. Only one I/O per analog I/O group must be defined as sampling capacitor. Note: These bits must not be modified when an acquisition is ongoing. Note: During the acquisition phase and even if the TSC peripheral alternate function is not enabled, as soon as the TSC_IOSCR bit is set, the corresponding GPIO analog switch is automatically controlled by the touch sensing controller.
    #[inline(always)]
    pub fn g2_io3(&self) -> G2_IO3_R {
        G2_IO3_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - Gx_IOy sampling mode These bits are set and cleared by software to configure the Gx_IOy as a sampling capacitor I/O. Only one I/O per analog I/O group must be defined as sampling capacitor. Note: These bits must not be modified when an acquisition is ongoing. Note: During the acquisition phase and even if the TSC peripheral alternate function is not enabled, as soon as the TSC_IOSCR bit is set, the corresponding GPIO analog switch is automatically controlled by the touch sensing controller.
    #[inline(always)]
    pub fn g2_io4(&self) -> G2_IO4_R {
        G2_IO4_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - Gx_IOy sampling mode These bits are set and cleared by software to configure the Gx_IOy as a sampling capacitor I/O. Only one I/O per analog I/O group must be defined as sampling capacitor. Note: These bits must not be modified when an acquisition is ongoing. Note: During the acquisition phase and even if the TSC peripheral alternate function is not enabled, as soon as the TSC_IOSCR bit is set, the corresponding GPIO analog switch is automatically controlled by the touch sensing controller.
    #[inline(always)]
    pub fn g3_io1(&self) -> G3_IO1_R {
        G3_IO1_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - Gx_IOy sampling mode These bits are set and cleared by software to configure the Gx_IOy as a sampling capacitor I/O. Only one I/O per analog I/O group must be defined as sampling capacitor. Note: These bits must not be modified when an acquisition is ongoing. Note: During the acquisition phase and even if the TSC peripheral alternate function is not enabled, as soon as the TSC_IOSCR bit is set, the corresponding GPIO analog switch is automatically controlled by the touch sensing controller.
    #[inline(always)]
    pub fn g3_io2(&self) -> G3_IO2_R {
        G3_IO2_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - Gx_IOy sampling mode These bits are set and cleared by software to configure the Gx_IOy as a sampling capacitor I/O. Only one I/O per analog I/O group must be defined as sampling capacitor. Note: These bits must not be modified when an acquisition is ongoing. Note: During the acquisition phase and even if the TSC peripheral alternate function is not enabled, as soon as the TSC_IOSCR bit is set, the corresponding GPIO analog switch is automatically controlled by the touch sensing controller.
    #[inline(always)]
    pub fn g3_io3(&self) -> G3_IO3_R {
        G3_IO3_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - Gx_IOy sampling mode These bits are set and cleared by software to configure the Gx_IOy as a sampling capacitor I/O. Only one I/O per analog I/O group must be defined as sampling capacitor. Note: These bits must not be modified when an acquisition is ongoing. Note: During the acquisition phase and even if the TSC peripheral alternate function is not enabled, as soon as the TSC_IOSCR bit is set, the corresponding GPIO analog switch is automatically controlled by the touch sensing controller.
    #[inline(always)]
    pub fn g3_io4(&self) -> G3_IO4_R {
        G3_IO4_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - Gx_IOy sampling mode These bits are set and cleared by software to configure the Gx_IOy as a sampling capacitor I/O. Only one I/O per analog I/O group must be defined as sampling capacitor. Note: These bits must not be modified when an acquisition is ongoing. Note: During the acquisition phase and even if the TSC peripheral alternate function is not enabled, as soon as the TSC_IOSCR bit is set, the corresponding GPIO analog switch is automatically controlled by the touch sensing controller.
    #[inline(always)]
    pub fn g4_io1(&self) -> G4_IO1_R {
        G4_IO1_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - Gx_IOy sampling mode These bits are set and cleared by software to configure the Gx_IOy as a sampling capacitor I/O. Only one I/O per analog I/O group must be defined as sampling capacitor. Note: These bits must not be modified when an acquisition is ongoing. Note: During the acquisition phase and even if the TSC peripheral alternate function is not enabled, as soon as the TSC_IOSCR bit is set, the corresponding GPIO analog switch is automatically controlled by the touch sensing controller.
    #[inline(always)]
    pub fn g4_io2(&self) -> G4_IO2_R {
        G4_IO2_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - Gx_IOy sampling mode These bits are set and cleared by software to configure the Gx_IOy as a sampling capacitor I/O. Only one I/O per analog I/O group must be defined as sampling capacitor. Note: These bits must not be modified when an acquisition is ongoing. Note: During the acquisition phase and even if the TSC peripheral alternate function is not enabled, as soon as the TSC_IOSCR bit is set, the corresponding GPIO analog switch is automatically controlled by the touch sensing controller.
    #[inline(always)]
    pub fn g4_io3(&self) -> G4_IO3_R {
        G4_IO3_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - Gx_IOy sampling mode These bits are set and cleared by software to configure the Gx_IOy as a sampling capacitor I/O. Only one I/O per analog I/O group must be defined as sampling capacitor. Note: These bits must not be modified when an acquisition is ongoing. Note: During the acquisition phase and even if the TSC peripheral alternate function is not enabled, as soon as the TSC_IOSCR bit is set, the corresponding GPIO analog switch is automatically controlled by the touch sensing controller.
    #[inline(always)]
    pub fn g4_io4(&self) -> G4_IO4_R {
        G4_IO4_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 16 - Gx_IOy sampling mode These bits are set and cleared by software to configure the Gx_IOy as a sampling capacitor I/O. Only one I/O per analog I/O group must be defined as sampling capacitor. Note: These bits must not be modified when an acquisition is ongoing. Note: During the acquisition phase and even if the TSC peripheral alternate function is not enabled, as soon as the TSC_IOSCR bit is set, the corresponding GPIO analog switch is automatically controlled by the touch sensing controller.
    #[inline(always)]
    pub fn g5_io1(&self) -> G5_IO1_R {
        G5_IO1_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - Gx_IOy sampling mode These bits are set and cleared by software to configure the Gx_IOy as a sampling capacitor I/O. Only one I/O per analog I/O group must be defined as sampling capacitor. Note: These bits must not be modified when an acquisition is ongoing. Note: During the acquisition phase and even if the TSC peripheral alternate function is not enabled, as soon as the TSC_IOSCR bit is set, the corresponding GPIO analog switch is automatically controlled by the touch sensing controller.
    #[inline(always)]
    pub fn g5_io2(&self) -> G5_IO2_R {
        G5_IO2_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - Gx_IOy sampling mode These bits are set and cleared by software to configure the Gx_IOy as a sampling capacitor I/O. Only one I/O per analog I/O group must be defined as sampling capacitor. Note: These bits must not be modified when an acquisition is ongoing. Note: During the acquisition phase and even if the TSC peripheral alternate function is not enabled, as soon as the TSC_IOSCR bit is set, the corresponding GPIO analog switch is automatically controlled by the touch sensing controller.
    #[inline(always)]
    pub fn g5_io3(&self) -> G5_IO3_R {
        G5_IO3_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 19 - Gx_IOy sampling mode These bits are set and cleared by software to configure the Gx_IOy as a sampling capacitor I/O. Only one I/O per analog I/O group must be defined as sampling capacitor. Note: These bits must not be modified when an acquisition is ongoing. Note: During the acquisition phase and even if the TSC peripheral alternate function is not enabled, as soon as the TSC_IOSCR bit is set, the corresponding GPIO analog switch is automatically controlled by the touch sensing controller.
    #[inline(always)]
    pub fn g5_io4(&self) -> G5_IO4_R {
        G5_IO4_R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 20 - Gx_IOy sampling mode These bits are set and cleared by software to configure the Gx_IOy as a sampling capacitor I/O. Only one I/O per analog I/O group must be defined as sampling capacitor. Note: These bits must not be modified when an acquisition is ongoing. Note: During the acquisition phase and even if the TSC peripheral alternate function is not enabled, as soon as the TSC_IOSCR bit is set, the corresponding GPIO analog switch is automatically controlled by the touch sensing controller.
    #[inline(always)]
    pub fn g6_io1(&self) -> G6_IO1_R {
        G6_IO1_R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 21 - Gx_IOy sampling mode These bits are set and cleared by software to configure the Gx_IOy as a sampling capacitor I/O. Only one I/O per analog I/O group must be defined as sampling capacitor. Note: These bits must not be modified when an acquisition is ongoing. Note: During the acquisition phase and even if the TSC peripheral alternate function is not enabled, as soon as the TSC_IOSCR bit is set, the corresponding GPIO analog switch is automatically controlled by the touch sensing controller.
    #[inline(always)]
    pub fn g6_io2(&self) -> G6_IO2_R {
        G6_IO2_R::new(((self.bits >> 21) & 1) != 0)
    }
    ///Bit 22 - Gx_IOy sampling mode These bits are set and cleared by software to configure the Gx_IOy as a sampling capacitor I/O. Only one I/O per analog I/O group must be defined as sampling capacitor. Note: These bits must not be modified when an acquisition is ongoing. Note: During the acquisition phase and even if the TSC peripheral alternate function is not enabled, as soon as the TSC_IOSCR bit is set, the corresponding GPIO analog switch is automatically controlled by the touch sensing controller.
    #[inline(always)]
    pub fn g6_io3(&self) -> G6_IO3_R {
        G6_IO3_R::new(((self.bits >> 22) & 1) != 0)
    }
    ///Bit 23 - Gx_IOy sampling mode These bits are set and cleared by software to configure the Gx_IOy as a sampling capacitor I/O. Only one I/O per analog I/O group must be defined as sampling capacitor. Note: These bits must not be modified when an acquisition is ongoing. Note: During the acquisition phase and even if the TSC peripheral alternate function is not enabled, as soon as the TSC_IOSCR bit is set, the corresponding GPIO analog switch is automatically controlled by the touch sensing controller.
    #[inline(always)]
    pub fn g6_io4(&self) -> G6_IO4_R {
        G6_IO4_R::new(((self.bits >> 23) & 1) != 0)
    }
    ///Bit 24 - Gx_IOy sampling mode These bits are set and cleared by software to configure the Gx_IOy as a sampling capacitor I/O. Only one I/O per analog I/O group must be defined as sampling capacitor. Note: These bits must not be modified when an acquisition is ongoing. Note: During the acquisition phase and even if the TSC peripheral alternate function is not enabled, as soon as the TSC_IOSCR bit is set, the corresponding GPIO analog switch is automatically controlled by the touch sensing controller.
    #[inline(always)]
    pub fn g7_io1(&self) -> G7_IO1_R {
        G7_IO1_R::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bit 25 - Gx_IOy sampling mode These bits are set and cleared by software to configure the Gx_IOy as a sampling capacitor I/O. Only one I/O per analog I/O group must be defined as sampling capacitor. Note: These bits must not be modified when an acquisition is ongoing. Note: During the acquisition phase and even if the TSC peripheral alternate function is not enabled, as soon as the TSC_IOSCR bit is set, the corresponding GPIO analog switch is automatically controlled by the touch sensing controller.
    #[inline(always)]
    pub fn g7_io2(&self) -> G7_IO2_R {
        G7_IO2_R::new(((self.bits >> 25) & 1) != 0)
    }
    ///Bit 26 - Gx_IOy sampling mode These bits are set and cleared by software to configure the Gx_IOy as a sampling capacitor I/O. Only one I/O per analog I/O group must be defined as sampling capacitor. Note: These bits must not be modified when an acquisition is ongoing. Note: During the acquisition phase and even if the TSC peripheral alternate function is not enabled, as soon as the TSC_IOSCR bit is set, the corresponding GPIO analog switch is automatically controlled by the touch sensing controller.
    #[inline(always)]
    pub fn g7_io3(&self) -> G7_IO3_R {
        G7_IO3_R::new(((self.bits >> 26) & 1) != 0)
    }
    ///Bit 27 - Gx_IOy sampling mode These bits are set and cleared by software to configure the Gx_IOy as a sampling capacitor I/O. Only one I/O per analog I/O group must be defined as sampling capacitor. Note: These bits must not be modified when an acquisition is ongoing. Note: During the acquisition phase and even if the TSC peripheral alternate function is not enabled, as soon as the TSC_IOSCR bit is set, the corresponding GPIO analog switch is automatically controlled by the touch sensing controller.
    #[inline(always)]
    pub fn g7_io4(&self) -> G7_IO4_R {
        G7_IO4_R::new(((self.bits >> 27) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TSC_IOSCR")
            .field("g1_io1", &self.g1_io1())
            .field("g1_io2", &self.g1_io2())
            .field("g1_io3", &self.g1_io3())
            .field("g1_io4", &self.g1_io4())
            .field("g2_io1", &self.g2_io1())
            .field("g2_io2", &self.g2_io2())
            .field("g2_io3", &self.g2_io3())
            .field("g2_io4", &self.g2_io4())
            .field("g3_io1", &self.g3_io1())
            .field("g3_io2", &self.g3_io2())
            .field("g3_io3", &self.g3_io3())
            .field("g3_io4", &self.g3_io4())
            .field("g4_io1", &self.g4_io1())
            .field("g4_io2", &self.g4_io2())
            .field("g4_io3", &self.g4_io3())
            .field("g4_io4", &self.g4_io4())
            .field("g5_io1", &self.g5_io1())
            .field("g5_io2", &self.g5_io2())
            .field("g5_io3", &self.g5_io3())
            .field("g5_io4", &self.g5_io4())
            .field("g6_io1", &self.g6_io1())
            .field("g6_io2", &self.g6_io2())
            .field("g6_io3", &self.g6_io3())
            .field("g6_io4", &self.g6_io4())
            .field("g7_io1", &self.g7_io1())
            .field("g7_io2", &self.g7_io2())
            .field("g7_io3", &self.g7_io3())
            .field("g7_io4", &self.g7_io4())
            .finish()
    }
}
impl W {
    ///Bit 0 - Gx_IOy sampling mode These bits are set and cleared by software to configure the Gx_IOy as a sampling capacitor I/O. Only one I/O per analog I/O group must be defined as sampling capacitor. Note: These bits must not be modified when an acquisition is ongoing. Note: During the acquisition phase and even if the TSC peripheral alternate function is not enabled, as soon as the TSC_IOSCR bit is set, the corresponding GPIO analog switch is automatically controlled by the touch sensing controller.
    #[inline(always)]
    pub fn g1_io1(&mut self) -> G1_IO1_W<TSC_IOSCRrs> {
        G1_IO1_W::new(self, 0)
    }
    ///Bit 1 - Gx_IOy sampling mode These bits are set and cleared by software to configure the Gx_IOy as a sampling capacitor I/O. Only one I/O per analog I/O group must be defined as sampling capacitor. Note: These bits must not be modified when an acquisition is ongoing. Note: During the acquisition phase and even if the TSC peripheral alternate function is not enabled, as soon as the TSC_IOSCR bit is set, the corresponding GPIO analog switch is automatically controlled by the touch sensing controller.
    #[inline(always)]
    pub fn g1_io2(&mut self) -> G1_IO2_W<TSC_IOSCRrs> {
        G1_IO2_W::new(self, 1)
    }
    ///Bit 2 - Gx_IOy sampling mode These bits are set and cleared by software to configure the Gx_IOy as a sampling capacitor I/O. Only one I/O per analog I/O group must be defined as sampling capacitor. Note: These bits must not be modified when an acquisition is ongoing. Note: During the acquisition phase and even if the TSC peripheral alternate function is not enabled, as soon as the TSC_IOSCR bit is set, the corresponding GPIO analog switch is automatically controlled by the touch sensing controller.
    #[inline(always)]
    pub fn g1_io3(&mut self) -> G1_IO3_W<TSC_IOSCRrs> {
        G1_IO3_W::new(self, 2)
    }
    ///Bit 3 - Gx_IOy sampling mode These bits are set and cleared by software to configure the Gx_IOy as a sampling capacitor I/O. Only one I/O per analog I/O group must be defined as sampling capacitor. Note: These bits must not be modified when an acquisition is ongoing. Note: During the acquisition phase and even if the TSC peripheral alternate function is not enabled, as soon as the TSC_IOSCR bit is set, the corresponding GPIO analog switch is automatically controlled by the touch sensing controller.
    #[inline(always)]
    pub fn g1_io4(&mut self) -> G1_IO4_W<TSC_IOSCRrs> {
        G1_IO4_W::new(self, 3)
    }
    ///Bit 4 - Gx_IOy sampling mode These bits are set and cleared by software to configure the Gx_IOy as a sampling capacitor I/O. Only one I/O per analog I/O group must be defined as sampling capacitor. Note: These bits must not be modified when an acquisition is ongoing. Note: During the acquisition phase and even if the TSC peripheral alternate function is not enabled, as soon as the TSC_IOSCR bit is set, the corresponding GPIO analog switch is automatically controlled by the touch sensing controller.
    #[inline(always)]
    pub fn g2_io1(&mut self) -> G2_IO1_W<TSC_IOSCRrs> {
        G2_IO1_W::new(self, 4)
    }
    ///Bit 5 - Gx_IOy sampling mode These bits are set and cleared by software to configure the Gx_IOy as a sampling capacitor I/O. Only one I/O per analog I/O group must be defined as sampling capacitor. Note: These bits must not be modified when an acquisition is ongoing. Note: During the acquisition phase and even if the TSC peripheral alternate function is not enabled, as soon as the TSC_IOSCR bit is set, the corresponding GPIO analog switch is automatically controlled by the touch sensing controller.
    #[inline(always)]
    pub fn g2_io2(&mut self) -> G2_IO2_W<TSC_IOSCRrs> {
        G2_IO2_W::new(self, 5)
    }
    ///Bit 6 - Gx_IOy sampling mode These bits are set and cleared by software to configure the Gx_IOy as a sampling capacitor I/O. Only one I/O per analog I/O group must be defined as sampling capacitor. Note: These bits must not be modified when an acquisition is ongoing. Note: During the acquisition phase and even if the TSC peripheral alternate function is not enabled, as soon as the TSC_IOSCR bit is set, the corresponding GPIO analog switch is automatically controlled by the touch sensing controller.
    #[inline(always)]
    pub fn g2_io3(&mut self) -> G2_IO3_W<TSC_IOSCRrs> {
        G2_IO3_W::new(self, 6)
    }
    ///Bit 7 - Gx_IOy sampling mode These bits are set and cleared by software to configure the Gx_IOy as a sampling capacitor I/O. Only one I/O per analog I/O group must be defined as sampling capacitor. Note: These bits must not be modified when an acquisition is ongoing. Note: During the acquisition phase and even if the TSC peripheral alternate function is not enabled, as soon as the TSC_IOSCR bit is set, the corresponding GPIO analog switch is automatically controlled by the touch sensing controller.
    #[inline(always)]
    pub fn g2_io4(&mut self) -> G2_IO4_W<TSC_IOSCRrs> {
        G2_IO4_W::new(self, 7)
    }
    ///Bit 8 - Gx_IOy sampling mode These bits are set and cleared by software to configure the Gx_IOy as a sampling capacitor I/O. Only one I/O per analog I/O group must be defined as sampling capacitor. Note: These bits must not be modified when an acquisition is ongoing. Note: During the acquisition phase and even if the TSC peripheral alternate function is not enabled, as soon as the TSC_IOSCR bit is set, the corresponding GPIO analog switch is automatically controlled by the touch sensing controller.
    #[inline(always)]
    pub fn g3_io1(&mut self) -> G3_IO1_W<TSC_IOSCRrs> {
        G3_IO1_W::new(self, 8)
    }
    ///Bit 9 - Gx_IOy sampling mode These bits are set and cleared by software to configure the Gx_IOy as a sampling capacitor I/O. Only one I/O per analog I/O group must be defined as sampling capacitor. Note: These bits must not be modified when an acquisition is ongoing. Note: During the acquisition phase and even if the TSC peripheral alternate function is not enabled, as soon as the TSC_IOSCR bit is set, the corresponding GPIO analog switch is automatically controlled by the touch sensing controller.
    #[inline(always)]
    pub fn g3_io2(&mut self) -> G3_IO2_W<TSC_IOSCRrs> {
        G3_IO2_W::new(self, 9)
    }
    ///Bit 10 - Gx_IOy sampling mode These bits are set and cleared by software to configure the Gx_IOy as a sampling capacitor I/O. Only one I/O per analog I/O group must be defined as sampling capacitor. Note: These bits must not be modified when an acquisition is ongoing. Note: During the acquisition phase and even if the TSC peripheral alternate function is not enabled, as soon as the TSC_IOSCR bit is set, the corresponding GPIO analog switch is automatically controlled by the touch sensing controller.
    #[inline(always)]
    pub fn g3_io3(&mut self) -> G3_IO3_W<TSC_IOSCRrs> {
        G3_IO3_W::new(self, 10)
    }
    ///Bit 11 - Gx_IOy sampling mode These bits are set and cleared by software to configure the Gx_IOy as a sampling capacitor I/O. Only one I/O per analog I/O group must be defined as sampling capacitor. Note: These bits must not be modified when an acquisition is ongoing. Note: During the acquisition phase and even if the TSC peripheral alternate function is not enabled, as soon as the TSC_IOSCR bit is set, the corresponding GPIO analog switch is automatically controlled by the touch sensing controller.
    #[inline(always)]
    pub fn g3_io4(&mut self) -> G3_IO4_W<TSC_IOSCRrs> {
        G3_IO4_W::new(self, 11)
    }
    ///Bit 12 - Gx_IOy sampling mode These bits are set and cleared by software to configure the Gx_IOy as a sampling capacitor I/O. Only one I/O per analog I/O group must be defined as sampling capacitor. Note: These bits must not be modified when an acquisition is ongoing. Note: During the acquisition phase and even if the TSC peripheral alternate function is not enabled, as soon as the TSC_IOSCR bit is set, the corresponding GPIO analog switch is automatically controlled by the touch sensing controller.
    #[inline(always)]
    pub fn g4_io1(&mut self) -> G4_IO1_W<TSC_IOSCRrs> {
        G4_IO1_W::new(self, 12)
    }
    ///Bit 13 - Gx_IOy sampling mode These bits are set and cleared by software to configure the Gx_IOy as a sampling capacitor I/O. Only one I/O per analog I/O group must be defined as sampling capacitor. Note: These bits must not be modified when an acquisition is ongoing. Note: During the acquisition phase and even if the TSC peripheral alternate function is not enabled, as soon as the TSC_IOSCR bit is set, the corresponding GPIO analog switch is automatically controlled by the touch sensing controller.
    #[inline(always)]
    pub fn g4_io2(&mut self) -> G4_IO2_W<TSC_IOSCRrs> {
        G4_IO2_W::new(self, 13)
    }
    ///Bit 14 - Gx_IOy sampling mode These bits are set and cleared by software to configure the Gx_IOy as a sampling capacitor I/O. Only one I/O per analog I/O group must be defined as sampling capacitor. Note: These bits must not be modified when an acquisition is ongoing. Note: During the acquisition phase and even if the TSC peripheral alternate function is not enabled, as soon as the TSC_IOSCR bit is set, the corresponding GPIO analog switch is automatically controlled by the touch sensing controller.
    #[inline(always)]
    pub fn g4_io3(&mut self) -> G4_IO3_W<TSC_IOSCRrs> {
        G4_IO3_W::new(self, 14)
    }
    ///Bit 15 - Gx_IOy sampling mode These bits are set and cleared by software to configure the Gx_IOy as a sampling capacitor I/O. Only one I/O per analog I/O group must be defined as sampling capacitor. Note: These bits must not be modified when an acquisition is ongoing. Note: During the acquisition phase and even if the TSC peripheral alternate function is not enabled, as soon as the TSC_IOSCR bit is set, the corresponding GPIO analog switch is automatically controlled by the touch sensing controller.
    #[inline(always)]
    pub fn g4_io4(&mut self) -> G4_IO4_W<TSC_IOSCRrs> {
        G4_IO4_W::new(self, 15)
    }
    ///Bit 16 - Gx_IOy sampling mode These bits are set and cleared by software to configure the Gx_IOy as a sampling capacitor I/O. Only one I/O per analog I/O group must be defined as sampling capacitor. Note: These bits must not be modified when an acquisition is ongoing. Note: During the acquisition phase and even if the TSC peripheral alternate function is not enabled, as soon as the TSC_IOSCR bit is set, the corresponding GPIO analog switch is automatically controlled by the touch sensing controller.
    #[inline(always)]
    pub fn g5_io1(&mut self) -> G5_IO1_W<TSC_IOSCRrs> {
        G5_IO1_W::new(self, 16)
    }
    ///Bit 17 - Gx_IOy sampling mode These bits are set and cleared by software to configure the Gx_IOy as a sampling capacitor I/O. Only one I/O per analog I/O group must be defined as sampling capacitor. Note: These bits must not be modified when an acquisition is ongoing. Note: During the acquisition phase and even if the TSC peripheral alternate function is not enabled, as soon as the TSC_IOSCR bit is set, the corresponding GPIO analog switch is automatically controlled by the touch sensing controller.
    #[inline(always)]
    pub fn g5_io2(&mut self) -> G5_IO2_W<TSC_IOSCRrs> {
        G5_IO2_W::new(self, 17)
    }
    ///Bit 18 - Gx_IOy sampling mode These bits are set and cleared by software to configure the Gx_IOy as a sampling capacitor I/O. Only one I/O per analog I/O group must be defined as sampling capacitor. Note: These bits must not be modified when an acquisition is ongoing. Note: During the acquisition phase and even if the TSC peripheral alternate function is not enabled, as soon as the TSC_IOSCR bit is set, the corresponding GPIO analog switch is automatically controlled by the touch sensing controller.
    #[inline(always)]
    pub fn g5_io3(&mut self) -> G5_IO3_W<TSC_IOSCRrs> {
        G5_IO3_W::new(self, 18)
    }
    ///Bit 19 - Gx_IOy sampling mode These bits are set and cleared by software to configure the Gx_IOy as a sampling capacitor I/O. Only one I/O per analog I/O group must be defined as sampling capacitor. Note: These bits must not be modified when an acquisition is ongoing. Note: During the acquisition phase and even if the TSC peripheral alternate function is not enabled, as soon as the TSC_IOSCR bit is set, the corresponding GPIO analog switch is automatically controlled by the touch sensing controller.
    #[inline(always)]
    pub fn g5_io4(&mut self) -> G5_IO4_W<TSC_IOSCRrs> {
        G5_IO4_W::new(self, 19)
    }
    ///Bit 20 - Gx_IOy sampling mode These bits are set and cleared by software to configure the Gx_IOy as a sampling capacitor I/O. Only one I/O per analog I/O group must be defined as sampling capacitor. Note: These bits must not be modified when an acquisition is ongoing. Note: During the acquisition phase and even if the TSC peripheral alternate function is not enabled, as soon as the TSC_IOSCR bit is set, the corresponding GPIO analog switch is automatically controlled by the touch sensing controller.
    #[inline(always)]
    pub fn g6_io1(&mut self) -> G6_IO1_W<TSC_IOSCRrs> {
        G6_IO1_W::new(self, 20)
    }
    ///Bit 21 - Gx_IOy sampling mode These bits are set and cleared by software to configure the Gx_IOy as a sampling capacitor I/O. Only one I/O per analog I/O group must be defined as sampling capacitor. Note: These bits must not be modified when an acquisition is ongoing. Note: During the acquisition phase and even if the TSC peripheral alternate function is not enabled, as soon as the TSC_IOSCR bit is set, the corresponding GPIO analog switch is automatically controlled by the touch sensing controller.
    #[inline(always)]
    pub fn g6_io2(&mut self) -> G6_IO2_W<TSC_IOSCRrs> {
        G6_IO2_W::new(self, 21)
    }
    ///Bit 22 - Gx_IOy sampling mode These bits are set and cleared by software to configure the Gx_IOy as a sampling capacitor I/O. Only one I/O per analog I/O group must be defined as sampling capacitor. Note: These bits must not be modified when an acquisition is ongoing. Note: During the acquisition phase and even if the TSC peripheral alternate function is not enabled, as soon as the TSC_IOSCR bit is set, the corresponding GPIO analog switch is automatically controlled by the touch sensing controller.
    #[inline(always)]
    pub fn g6_io3(&mut self) -> G6_IO3_W<TSC_IOSCRrs> {
        G6_IO3_W::new(self, 22)
    }
    ///Bit 23 - Gx_IOy sampling mode These bits are set and cleared by software to configure the Gx_IOy as a sampling capacitor I/O. Only one I/O per analog I/O group must be defined as sampling capacitor. Note: These bits must not be modified when an acquisition is ongoing. Note: During the acquisition phase and even if the TSC peripheral alternate function is not enabled, as soon as the TSC_IOSCR bit is set, the corresponding GPIO analog switch is automatically controlled by the touch sensing controller.
    #[inline(always)]
    pub fn g6_io4(&mut self) -> G6_IO4_W<TSC_IOSCRrs> {
        G6_IO4_W::new(self, 23)
    }
    ///Bit 24 - Gx_IOy sampling mode These bits are set and cleared by software to configure the Gx_IOy as a sampling capacitor I/O. Only one I/O per analog I/O group must be defined as sampling capacitor. Note: These bits must not be modified when an acquisition is ongoing. Note: During the acquisition phase and even if the TSC peripheral alternate function is not enabled, as soon as the TSC_IOSCR bit is set, the corresponding GPIO analog switch is automatically controlled by the touch sensing controller.
    #[inline(always)]
    pub fn g7_io1(&mut self) -> G7_IO1_W<TSC_IOSCRrs> {
        G7_IO1_W::new(self, 24)
    }
    ///Bit 25 - Gx_IOy sampling mode These bits are set and cleared by software to configure the Gx_IOy as a sampling capacitor I/O. Only one I/O per analog I/O group must be defined as sampling capacitor. Note: These bits must not be modified when an acquisition is ongoing. Note: During the acquisition phase and even if the TSC peripheral alternate function is not enabled, as soon as the TSC_IOSCR bit is set, the corresponding GPIO analog switch is automatically controlled by the touch sensing controller.
    #[inline(always)]
    pub fn g7_io2(&mut self) -> G7_IO2_W<TSC_IOSCRrs> {
        G7_IO2_W::new(self, 25)
    }
    ///Bit 26 - Gx_IOy sampling mode These bits are set and cleared by software to configure the Gx_IOy as a sampling capacitor I/O. Only one I/O per analog I/O group must be defined as sampling capacitor. Note: These bits must not be modified when an acquisition is ongoing. Note: During the acquisition phase and even if the TSC peripheral alternate function is not enabled, as soon as the TSC_IOSCR bit is set, the corresponding GPIO analog switch is automatically controlled by the touch sensing controller.
    #[inline(always)]
    pub fn g7_io3(&mut self) -> G7_IO3_W<TSC_IOSCRrs> {
        G7_IO3_W::new(self, 26)
    }
    ///Bit 27 - Gx_IOy sampling mode These bits are set and cleared by software to configure the Gx_IOy as a sampling capacitor I/O. Only one I/O per analog I/O group must be defined as sampling capacitor. Note: These bits must not be modified when an acquisition is ongoing. Note: During the acquisition phase and even if the TSC peripheral alternate function is not enabled, as soon as the TSC_IOSCR bit is set, the corresponding GPIO analog switch is automatically controlled by the touch sensing controller.
    #[inline(always)]
    pub fn g7_io4(&mut self) -> G7_IO4_W<TSC_IOSCRrs> {
        G7_IO4_W::new(self, 27)
    }
}
/**TSC I/O sampling control register

You can [`read`](crate::Reg::read) this register and get [`tsc_ioscr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tsc_ioscr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U031.html#TSC:TSC_IOSCR)*/
pub struct TSC_IOSCRrs;
impl crate::RegisterSpec for TSC_IOSCRrs {
    type Ux = u32;
}
///`read()` method returns [`tsc_ioscr::R`](R) reader structure
impl crate::Readable for TSC_IOSCRrs {}
///`write(|w| ..)` method takes [`tsc_ioscr::W`](W) writer structure
impl crate::Writable for TSC_IOSCRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets TSC_IOSCR to value 0
impl crate::Resettable for TSC_IOSCRrs {
    const RESET_VALUE: u32 = 0;
}
