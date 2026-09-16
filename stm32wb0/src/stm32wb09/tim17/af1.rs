///Register `AF1` reader
pub type R = crate::R<AF1rs>;
///Register `AF1` writer
pub type W = crate::W<AF1rs>;
///Field `BKINE` reader - BKINE: BRK BKIN enable. This bit enables the BKIN alternate function input for the timer's BRK input. BKIN input is ORed with the other enabled BRK sources. 0: BKIN input disabled. 1: BKIN input enabled. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register)
pub type BKINE_R = crate::BitReader;
///Field `BKINE` writer - BKINE: BRK BKIN enable. This bit enables the BKIN alternate function input for the timer's BRK input. BKIN input is ORed with the other enabled BRK sources. 0: BKIN input disabled. 1: BKIN input enabled. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register)
pub type BKINE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `BKCMP1E` reader - BKCMP1E: BRK COMP1 enable. This bit enables the COMP1 for the timer's BRK input. COMP1 output is ORed with the other enabled BRK sources. 0: COMP1 input disabled. 1: COMP1 input enabled. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register)
pub type BKCMP1E_R = crate::BitReader;
///Field `BKCMP1E` writer - BKCMP1E: BRK COMP1 enable. This bit enables the COMP1 for the timer's BRK input. COMP1 output is ORed with the other enabled BRK sources. 0: COMP1 input disabled. 1: COMP1 input enabled. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register)
pub type BKCMP1E_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `BKCMP2E` reader - BKCMP1E: BRK COMP1 enable. This bit enables the COMP1 for the timer's BRK input. COMP1 output is ORed with the other enabled BRK sources. 0: COMP1 input disabled. 1: COMP1 input enabled. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register)
pub type BKCMP2E_R = crate::BitReader;
///Field `BKCMP2E` writer - BKCMP1E: BRK COMP1 enable. This bit enables the COMP1 for the timer's BRK input. COMP1 output is ORed with the other enabled BRK sources. 0: COMP1 input disabled. 1: COMP1 input enabled. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register)
pub type BKCMP2E_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `AF1_8_3` reader - AF1\[13:12\] Not used in Blue51. Not available in IUM
pub type AF1_8_3_R = crate::FieldReader;
///Field `AF1_8_3` writer - AF1\[13:12\] Not used in Blue51. Not available in IUM
pub type AF1_8_3_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
///Field `BKINP` reader - BKINP: BRK BKIN input polarity. This bit selects the BKIN alternate function input sensitivity. It must be programmed together with the BKP polarity bit. 0: BKIN input is active low. 1: BKIN input is active high. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register)
pub type BKINP_R = crate::BitReader;
///Field `BKINP` writer - BKINP: BRK BKIN input polarity. This bit selects the BKIN alternate function input sensitivity. It must be programmed together with the BKP polarity bit. 0: BKIN input is active low. 1: BKIN input is active high. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register)
pub type BKINP_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `BKCMP1P` reader - BKCMP1P: BRK COMP1 input polarity. This bit selects the COMP1 input sensitivity. It must be programmed together with the BKP polarity bit. 0: COMP1 input is active low. 1: COMP1 input is active high. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register)
pub type BKCMP1P_R = crate::BitReader;
///Field `BKCMP1P` writer - BKCMP1P: BRK COMP1 input polarity. This bit selects the COMP1 input sensitivity. It must be programmed together with the BKP polarity bit. 0: COMP1 input is active low. 1: COMP1 input is active high. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register)
pub type BKCMP1P_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `BKCMP2P` reader - BKCMP2P: BRK COMP2 input polarity. This bit selects the COMP2 input sensitivity. It must be programmed together with the BKP polarity bit. 0: COMP2 input is active low. 1: COMP2 input is active high. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register)
pub type BKCMP2P_R = crate::BitReader;
///Field `BKCMP2P` writer - BKCMP2P: BRK COMP2 input polarity. This bit selects the COMP2 input sensitivity. It must be programmed together with the BKP polarity bit. 0: COMP2 input is active low. 1: COMP2 input is active high. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register)
pub type BKCMP2P_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `AF1_13_12` reader - AF1\[13:12\] Not used in Blue51. Not available in IUM
pub type AF1_13_12_R = crate::FieldReader;
///Field `AF1_13_12` writer - AF1\[13:12\] Not used in Blue51. Not available in IUM
pub type AF1_13_12_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    ///Bit 0 - BKINE: BRK BKIN enable. This bit enables the BKIN alternate function input for the timer's BRK input. BKIN input is ORed with the other enabled BRK sources. 0: BKIN input disabled. 1: BKIN input enabled. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register)
    #[inline(always)]
    pub fn bkine(&self) -> BKINE_R {
        BKINE_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - BKCMP1E: BRK COMP1 enable. This bit enables the COMP1 for the timer's BRK input. COMP1 output is ORed with the other enabled BRK sources. 0: COMP1 input disabled. 1: COMP1 input enabled. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register)
    #[inline(always)]
    pub fn bkcmp1e(&self) -> BKCMP1E_R {
        BKCMP1E_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - BKCMP1E: BRK COMP1 enable. This bit enables the COMP1 for the timer's BRK input. COMP1 output is ORed with the other enabled BRK sources. 0: COMP1 input disabled. 1: COMP1 input enabled. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register)
    #[inline(always)]
    pub fn bkcmp2e(&self) -> BKCMP2E_R {
        BKCMP2E_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bits 3:8 - AF1\[13:12\] Not used in Blue51. Not available in IUM
    #[inline(always)]
    pub fn af1_8_3(&self) -> AF1_8_3_R {
        AF1_8_3_R::new(((self.bits >> 3) & 0x3f) as u8)
    }
    ///Bit 9 - BKINP: BRK BKIN input polarity. This bit selects the BKIN alternate function input sensitivity. It must be programmed together with the BKP polarity bit. 0: BKIN input is active low. 1: BKIN input is active high. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register)
    #[inline(always)]
    pub fn bkinp(&self) -> BKINP_R {
        BKINP_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - BKCMP1P: BRK COMP1 input polarity. This bit selects the COMP1 input sensitivity. It must be programmed together with the BKP polarity bit. 0: COMP1 input is active low. 1: COMP1 input is active high. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register)
    #[inline(always)]
    pub fn bkcmp1p(&self) -> BKCMP1P_R {
        BKCMP1P_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - BKCMP2P: BRK COMP2 input polarity. This bit selects the COMP2 input sensitivity. It must be programmed together with the BKP polarity bit. 0: COMP2 input is active low. 1: COMP2 input is active high. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register)
    #[inline(always)]
    pub fn bkcmp2p(&self) -> BKCMP2P_R {
        BKCMP2P_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bits 12:13 - AF1\[13:12\] Not used in Blue51. Not available in IUM
    #[inline(always)]
    pub fn af1_13_12(&self) -> AF1_13_12_R {
        AF1_13_12_R::new(((self.bits >> 12) & 3) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AF1")
            .field("bkine", &self.bkine())
            .field("bkcmp1e", &self.bkcmp1e())
            .field("bkcmp2e", &self.bkcmp2e())
            .field("af1_8_3", &self.af1_8_3())
            .field("bkinp", &self.bkinp())
            .field("bkcmp1p", &self.bkcmp1p())
            .field("bkcmp2p", &self.bkcmp2p())
            .field("af1_13_12", &self.af1_13_12())
            .finish()
    }
}
impl W {
    ///Bit 0 - BKINE: BRK BKIN enable. This bit enables the BKIN alternate function input for the timer's BRK input. BKIN input is ORed with the other enabled BRK sources. 0: BKIN input disabled. 1: BKIN input enabled. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register)
    #[inline(always)]
    pub fn bkine(&mut self) -> BKINE_W<'_, AF1rs> {
        BKINE_W::new(self, 0)
    }
    ///Bit 1 - BKCMP1E: BRK COMP1 enable. This bit enables the COMP1 for the timer's BRK input. COMP1 output is ORed with the other enabled BRK sources. 0: COMP1 input disabled. 1: COMP1 input enabled. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register)
    #[inline(always)]
    pub fn bkcmp1e(&mut self) -> BKCMP1E_W<'_, AF1rs> {
        BKCMP1E_W::new(self, 1)
    }
    ///Bit 2 - BKCMP1E: BRK COMP1 enable. This bit enables the COMP1 for the timer's BRK input. COMP1 output is ORed with the other enabled BRK sources. 0: COMP1 input disabled. 1: COMP1 input enabled. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register)
    #[inline(always)]
    pub fn bkcmp2e(&mut self) -> BKCMP2E_W<'_, AF1rs> {
        BKCMP2E_W::new(self, 2)
    }
    ///Bits 3:8 - AF1\[13:12\] Not used in Blue51. Not available in IUM
    #[inline(always)]
    pub fn af1_8_3(&mut self) -> AF1_8_3_W<'_, AF1rs> {
        AF1_8_3_W::new(self, 3)
    }
    ///Bit 9 - BKINP: BRK BKIN input polarity. This bit selects the BKIN alternate function input sensitivity. It must be programmed together with the BKP polarity bit. 0: BKIN input is active low. 1: BKIN input is active high. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register)
    #[inline(always)]
    pub fn bkinp(&mut self) -> BKINP_W<'_, AF1rs> {
        BKINP_W::new(self, 9)
    }
    ///Bit 10 - BKCMP1P: BRK COMP1 input polarity. This bit selects the COMP1 input sensitivity. It must be programmed together with the BKP polarity bit. 0: COMP1 input is active low. 1: COMP1 input is active high. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register)
    #[inline(always)]
    pub fn bkcmp1p(&mut self) -> BKCMP1P_W<'_, AF1rs> {
        BKCMP1P_W::new(self, 10)
    }
    ///Bit 11 - BKCMP2P: BRK COMP2 input polarity. This bit selects the COMP2 input sensitivity. It must be programmed together with the BKP polarity bit. 0: COMP2 input is active low. 1: COMP2 input is active high. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register)
    #[inline(always)]
    pub fn bkcmp2p(&mut self) -> BKCMP2P_W<'_, AF1rs> {
        BKCMP2P_W::new(self, 11)
    }
    ///Bits 12:13 - AF1\[13:12\] Not used in Blue51. Not available in IUM
    #[inline(always)]
    pub fn af1_13_12(&mut self) -> AF1_13_12_W<'_, AF1rs> {
        AF1_13_12_W::new(self, 12)
    }
}
/**AF1 register

You can [`read`](crate::Reg::read) this register and get [`af1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`af1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB09.html#TIM17:AF1)*/
pub struct AF1rs;
impl crate::RegisterSpec for AF1rs {
    type Ux = u32;
}
///`read()` method returns [`af1::R`](R) reader structure
impl crate::Readable for AF1rs {}
///`write(|w| ..)` method takes [`af1::W`](W) writer structure
impl crate::Writable for AF1rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets AF1 to value 0x01
impl crate::Resettable for AF1rs {
    const RESET_VALUE: u32 = 0x01;
}
