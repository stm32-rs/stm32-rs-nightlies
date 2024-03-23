#[doc = "Register `AF1` reader"]
pub type R = crate::R<AF1rs>;
#[doc = "Register `AF1` writer"]
pub type W = crate::W<AF1rs>;
#[doc = "Field `BKINE` reader - BRK BKIN input enable This bit enables the BKIN alternate function input for the timerâ\u{80}\u{99}s BRK input. BKIN input is 'ORedâ\u{80}\u{99} with the other BRK sources. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register)."]
pub type BKINE_R = crate::BitReader;
#[doc = "Field `BKINE` writer - BRK BKIN input enable This bit enables the BKIN alternate function input for the timerâ\u{80}\u{99}s BRK input. BKIN input is 'ORedâ\u{80}\u{99} with the other BRK sources. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register)."]
pub type BKINE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BKCMP1E` reader - BRK COMP1 enable This bit enables the COMP1 for the timerâ\u{80}\u{99}s BRK input. COMP1 output is 'ORedâ\u{80}\u{99} with the other BRK sources. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register)."]
pub type BKCMP1E_R = crate::BitReader;
#[doc = "Field `BKCMP1E` writer - BRK COMP1 enable This bit enables the COMP1 for the timerâ\u{80}\u{99}s BRK input. COMP1 output is 'ORedâ\u{80}\u{99} with the other BRK sources. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register)."]
pub type BKCMP1E_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BKCMP2E` reader - BRK COMP2 enable This bit enables the COMP2 for the timerâ\u{80}\u{99}s BRK input. COMP2 output is 'ORedâ\u{80}\u{99} with the other BRK sources. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register)."]
pub type BKCMP2E_R = crate::BitReader;
#[doc = "Field `BKCMP2E` writer - BRK COMP2 enable This bit enables the COMP2 for the timerâ\u{80}\u{99}s BRK input. COMP2 output is 'ORedâ\u{80}\u{99} with the other BRK sources. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register)."]
pub type BKCMP2E_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BKCMP3E` reader - BRK COMP3 enable This bit enables the COMP3 for the timerâ\u{80}\u{99}s BRK input. COMP3 output is 'ORedâ\u{80}\u{99} with the other BRK sources. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register)."]
pub type BKCMP3E_R = crate::BitReader;
#[doc = "Field `BKCMP3E` writer - BRK COMP3 enable This bit enables the COMP3 for the timerâ\u{80}\u{99}s BRK input. COMP3 output is 'ORedâ\u{80}\u{99} with the other BRK sources. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register)."]
pub type BKCMP3E_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BKINP` reader - BRK BKIN input polarity This bit selects the BKIN alternate function input sensitivity. It must be programmed together with the BKP polarity bit. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register)."]
pub type BKINP_R = crate::BitReader;
#[doc = "Field `BKINP` writer - BRK BKIN input polarity This bit selects the BKIN alternate function input sensitivity. It must be programmed together with the BKP polarity bit. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register)."]
pub type BKINP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BKCMP1P` reader - BRK COMP1 input polarity This bit selects the COMP1 input sensitivity. It must be programmed together with the BKP polarity bit. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register)."]
pub type BKCMP1P_R = crate::BitReader;
#[doc = "Field `BKCMP1P` writer - BRK COMP1 input polarity This bit selects the COMP1 input sensitivity. It must be programmed together with the BKP polarity bit. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register)."]
pub type BKCMP1P_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BKCMP2P` reader - BRK COMP2 input polarity This bit selects the COMP2 input sensitivity. It must be programmed together with the BKP polarity bit. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register)."]
pub type BKCMP2P_R = crate::BitReader;
#[doc = "Field `BKCMP2P` writer - BRK COMP2 input polarity This bit selects the COMP2 input sensitivity. It must be programmed together with the BKP polarity bit. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register)."]
pub type BKCMP2P_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BKCMP3P` reader - BRK COMP3 input polarity This bit selects the COMP3 input sensitivity. It must be programmed together with the BKP polarity bit. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register)."]
pub type BKCMP3P_R = crate::BitReader;
#[doc = "Field `BKCMP3P` writer - BRK COMP3 input polarity This bit selects the COMP3 input sensitivity. It must be programmed together with the BKP polarity bit. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register)."]
pub type BKCMP3P_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - BRK BKIN input enable This bit enables the BKIN alternate function input for the timerâ\u{80}\u{99}s BRK input. BKIN input is 'ORedâ\u{80}\u{99} with the other BRK sources. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register)."]
    #[inline(always)]
    pub fn bkine(&self) -> BKINE_R {
        BKINE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - BRK COMP1 enable This bit enables the COMP1 for the timerâ\u{80}\u{99}s BRK input. COMP1 output is 'ORedâ\u{80}\u{99} with the other BRK sources. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register)."]
    #[inline(always)]
    pub fn bkcmp1e(&self) -> BKCMP1E_R {
        BKCMP1E_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - BRK COMP2 enable This bit enables the COMP2 for the timerâ\u{80}\u{99}s BRK input. COMP2 output is 'ORedâ\u{80}\u{99} with the other BRK sources. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register)."]
    #[inline(always)]
    pub fn bkcmp2e(&self) -> BKCMP2E_R {
        BKCMP2E_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - BRK COMP3 enable This bit enables the COMP3 for the timerâ\u{80}\u{99}s BRK input. COMP3 output is 'ORedâ\u{80}\u{99} with the other BRK sources. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register)."]
    #[inline(always)]
    pub fn bkcmp3e(&self) -> BKCMP3E_R {
        BKCMP3E_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 9 - BRK BKIN input polarity This bit selects the BKIN alternate function input sensitivity. It must be programmed together with the BKP polarity bit. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register)."]
    #[inline(always)]
    pub fn bkinp(&self) -> BKINP_R {
        BKINP_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - BRK COMP1 input polarity This bit selects the COMP1 input sensitivity. It must be programmed together with the BKP polarity bit. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register)."]
    #[inline(always)]
    pub fn bkcmp1p(&self) -> BKCMP1P_R {
        BKCMP1P_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - BRK COMP2 input polarity This bit selects the COMP2 input sensitivity. It must be programmed together with the BKP polarity bit. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register)."]
    #[inline(always)]
    pub fn bkcmp2p(&self) -> BKCMP2P_R {
        BKCMP2P_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - BRK COMP3 input polarity This bit selects the COMP3 input sensitivity. It must be programmed together with the BKP polarity bit. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register)."]
    #[inline(always)]
    pub fn bkcmp3p(&self) -> BKCMP3P_R {
        BKCMP3P_R::new(((self.bits >> 12) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - BRK BKIN input enable This bit enables the BKIN alternate function input for the timerâ\u{80}\u{99}s BRK input. BKIN input is 'ORedâ\u{80}\u{99} with the other BRK sources. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register)."]
    #[inline(always)]
    #[must_use]
    pub fn bkine(&mut self) -> BKINE_W<AF1rs> {
        BKINE_W::new(self, 0)
    }
    #[doc = "Bit 1 - BRK COMP1 enable This bit enables the COMP1 for the timerâ\u{80}\u{99}s BRK input. COMP1 output is 'ORedâ\u{80}\u{99} with the other BRK sources. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register)."]
    #[inline(always)]
    #[must_use]
    pub fn bkcmp1e(&mut self) -> BKCMP1E_W<AF1rs> {
        BKCMP1E_W::new(self, 1)
    }
    #[doc = "Bit 2 - BRK COMP2 enable This bit enables the COMP2 for the timerâ\u{80}\u{99}s BRK input. COMP2 output is 'ORedâ\u{80}\u{99} with the other BRK sources. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register)."]
    #[inline(always)]
    #[must_use]
    pub fn bkcmp2e(&mut self) -> BKCMP2E_W<AF1rs> {
        BKCMP2E_W::new(self, 2)
    }
    #[doc = "Bit 3 - BRK COMP3 enable This bit enables the COMP3 for the timerâ\u{80}\u{99}s BRK input. COMP3 output is 'ORedâ\u{80}\u{99} with the other BRK sources. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register)."]
    #[inline(always)]
    #[must_use]
    pub fn bkcmp3e(&mut self) -> BKCMP3E_W<AF1rs> {
        BKCMP3E_W::new(self, 3)
    }
    #[doc = "Bit 9 - BRK BKIN input polarity This bit selects the BKIN alternate function input sensitivity. It must be programmed together with the BKP polarity bit. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register)."]
    #[inline(always)]
    #[must_use]
    pub fn bkinp(&mut self) -> BKINP_W<AF1rs> {
        BKINP_W::new(self, 9)
    }
    #[doc = "Bit 10 - BRK COMP1 input polarity This bit selects the COMP1 input sensitivity. It must be programmed together with the BKP polarity bit. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register)."]
    #[inline(always)]
    #[must_use]
    pub fn bkcmp1p(&mut self) -> BKCMP1P_W<AF1rs> {
        BKCMP1P_W::new(self, 10)
    }
    #[doc = "Bit 11 - BRK COMP2 input polarity This bit selects the COMP2 input sensitivity. It must be programmed together with the BKP polarity bit. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register)."]
    #[inline(always)]
    #[must_use]
    pub fn bkcmp2p(&mut self) -> BKCMP2P_W<AF1rs> {
        BKCMP2P_W::new(self, 11)
    }
    #[doc = "Bit 12 - BRK COMP3 input polarity This bit selects the COMP3 input sensitivity. It must be programmed together with the BKP polarity bit. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register)."]
    #[inline(always)]
    #[must_use]
    pub fn bkcmp3p(&mut self) -> BKCMP3P_W<AF1rs> {
        BKCMP3P_W::new(self, 12)
    }
}
#[doc = "TIM15 alternate register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`af1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`af1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AF1rs;
impl crate::RegisterSpec for AF1rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`af1::R`](R) reader structure"]
impl crate::Readable for AF1rs {}
#[doc = "`write(|w| ..)` method takes [`af1::W`](W) writer structure"]
impl crate::Writable for AF1rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets AF1 to value 0x01"]
impl crate::Resettable for AF1rs {
    const RESET_VALUE: u32 = 0x01;
}
