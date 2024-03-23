#[doc = "Register `TIM1_AF1` reader"]
pub type R = crate::R<TIM1_AF1rs>;
#[doc = "Register `TIM1_AF1` writer"]
pub type W = crate::W<TIM1_AF1rs>;
#[doc = "Field `BKINE` reader - TIMx_BKIN input enable This bit enables the TIMx_BKIN alternate function input for the timerâ\u{80}\u{99}s tim_brk input. TIMx_BKIN input is 'ORedâ\u{80}\u{99} with the other tim_brk sources. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register)."]
pub type BKINE_R = crate::BitReader;
#[doc = "Field `BKINE` writer - TIMx_BKIN input enable This bit enables the TIMx_BKIN alternate function input for the timerâ\u{80}\u{99}s tim_brk input. TIMx_BKIN input is 'ORedâ\u{80}\u{99} with the other tim_brk sources. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register)."]
pub type BKINE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BKCMP1E` reader - tim_brk_cmp1 enable This bit enables the tim_brk_cmp1 for the timerâ\u{80}\u{99}s tim_brk input. tim_brk_cmp1 output is 'ORedâ\u{80}\u{99} with the other tim_brk sources. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register)."]
pub type BKCMP1E_R = crate::BitReader;
#[doc = "Field `BKCMP1E` writer - tim_brk_cmp1 enable This bit enables the tim_brk_cmp1 for the timerâ\u{80}\u{99}s tim_brk input. tim_brk_cmp1 output is 'ORedâ\u{80}\u{99} with the other tim_brk sources. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register)."]
pub type BKCMP1E_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BKCMP2E` reader - tim_brk_cmp2 enable This bit enables the tim_brk_cmp2 for the timerâ\u{80}\u{99}s tim_brk input. tim_brk_cmp2 output is 'ORedâ\u{80}\u{99} with the other tim_brk sources. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register)."]
pub type BKCMP2E_R = crate::BitReader;
#[doc = "Field `BKCMP2E` writer - tim_brk_cmp2 enable This bit enables the tim_brk_cmp2 for the timerâ\u{80}\u{99}s tim_brk input. tim_brk_cmp2 output is 'ORedâ\u{80}\u{99} with the other tim_brk sources. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register)."]
pub type BKCMP2E_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BKCMP3E` reader - tim_brk_cmp3 enable This bit enables the tim_brk_cmp3 for the timerâ\u{80}\u{99}s tim_brk input. tim_brk_cmp3 output is 'ORedâ\u{80}\u{99} with the other tim_brk sources. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register)."]
pub type BKCMP3E_R = crate::BitReader;
#[doc = "Field `BKCMP3E` writer - tim_brk_cmp3 enable This bit enables the tim_brk_cmp3 for the timerâ\u{80}\u{99}s tim_brk input. tim_brk_cmp3 output is 'ORedâ\u{80}\u{99} with the other tim_brk sources. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register)."]
pub type BKCMP3E_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BKCMP4E` reader - tim_brk_cmp4 enable This bit enables the tim_brk_cmp4 for the timerâ\u{80}\u{99}s tim_brk input. tim_brk_cmp4 output is 'ORedâ\u{80}\u{99} with the other tim_brk sources. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register)."]
pub type BKCMP4E_R = crate::BitReader;
#[doc = "Field `BKCMP4E` writer - tim_brk_cmp4 enable This bit enables the tim_brk_cmp4 for the timerâ\u{80}\u{99}s tim_brk input. tim_brk_cmp4 output is 'ORedâ\u{80}\u{99} with the other tim_brk sources. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register)."]
pub type BKCMP4E_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BKCMP5E` reader - tim_brk_cmp5 enable This bit enables the tim_brk_cmp5 for the timerâ\u{80}\u{99}s tim_brk input. tim_brk_cmp5 output is 'ORedâ\u{80}\u{99} with the other tim_brk sources. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register)."]
pub type BKCMP5E_R = crate::BitReader;
#[doc = "Field `BKCMP5E` writer - tim_brk_cmp5 enable This bit enables the tim_brk_cmp5 for the timerâ\u{80}\u{99}s tim_brk input. tim_brk_cmp5 output is 'ORedâ\u{80}\u{99} with the other tim_brk sources. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register)."]
pub type BKCMP5E_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BKCMP6E` reader - tim_brk_cmp6 enable This bit enables the tim_brk_cmp6 for the timerâ\u{80}\u{99}s tim_brk input. tim_brk_cmp6 output is 'ORedâ\u{80}\u{99} with the other tim_brk sources. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register)."]
pub type BKCMP6E_R = crate::BitReader;
#[doc = "Field `BKCMP6E` writer - tim_brk_cmp6 enable This bit enables the tim_brk_cmp6 for the timerâ\u{80}\u{99}s tim_brk input. tim_brk_cmp6 output is 'ORedâ\u{80}\u{99} with the other tim_brk sources. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register)."]
pub type BKCMP6E_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BKCMP7E` reader - tim_brk_cmp7 enable This bit enables the tim_brk_cmp7 for the timerâ\u{80}\u{99}s tim_brk input. tim_brk_cmp7 output is 'ORedâ\u{80}\u{99} with the other tim_brk sources. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register)."]
pub type BKCMP7E_R = crate::BitReader;
#[doc = "Field `BKCMP7E` writer - tim_brk_cmp7 enable This bit enables the tim_brk_cmp7 for the timerâ\u{80}\u{99}s tim_brk input. tim_brk_cmp7 output is 'ORedâ\u{80}\u{99} with the other tim_brk sources. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register)."]
pub type BKCMP7E_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BKCMP8E` reader - tim_brk_cmp8 enable This bit enables the tim_brk_cmp8 for the timerâ\u{80}\u{99}s tim_brk input. tim_brk_cmp8 output is 'ORedâ\u{80}\u{99} with the other tim_brk sources. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register)."]
pub type BKCMP8E_R = crate::BitReader;
#[doc = "Field `BKCMP8E` writer - tim_brk_cmp8 enable This bit enables the tim_brk_cmp8 for the timerâ\u{80}\u{99}s tim_brk input. tim_brk_cmp8 output is 'ORedâ\u{80}\u{99} with the other tim_brk sources. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register)."]
pub type BKCMP8E_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BKINP` reader - TIMx_BKIN input polarity This bit selects the TIMx_BKIN alternate function input sensitivity. It must be programmed together with the BKP polarity bit. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register)."]
pub type BKINP_R = crate::BitReader;
#[doc = "Field `BKINP` writer - TIMx_BKIN input polarity This bit selects the TIMx_BKIN alternate function input sensitivity. It must be programmed together with the BKP polarity bit. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register)."]
pub type BKINP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BKCMP1P` reader - tim_brk_cmp1 input polarity This bit selects the tim_brk_cmp1 input sensitivity. It must be programmed together with the BKP polarity bit. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register)."]
pub type BKCMP1P_R = crate::BitReader;
#[doc = "Field `BKCMP1P` writer - tim_brk_cmp1 input polarity This bit selects the tim_brk_cmp1 input sensitivity. It must be programmed together with the BKP polarity bit. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register)."]
pub type BKCMP1P_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BKCMP2P` reader - tim_brk_cmp2 input polarity This bit selects the tim_brk_cmp2 input sensitivity. It must be programmed together with the BKP polarity bit. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register)."]
pub type BKCMP2P_R = crate::BitReader;
#[doc = "Field `BKCMP2P` writer - tim_brk_cmp2 input polarity This bit selects the tim_brk_cmp2 input sensitivity. It must be programmed together with the BKP polarity bit. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register)."]
pub type BKCMP2P_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BKCMP3P` reader - tim_brk_cmp3 input polarity This bit selects the tim_brk_cmp3 input sensitivity. It must be programmed together with the BKP polarity bit. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register)."]
pub type BKCMP3P_R = crate::BitReader;
#[doc = "Field `BKCMP3P` writer - tim_brk_cmp3 input polarity This bit selects the tim_brk_cmp3 input sensitivity. It must be programmed together with the BKP polarity bit. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register)."]
pub type BKCMP3P_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BKCMP4P` reader - tim_brk_cmp4 input polarity This bit selects the tim_brk_cmp4 input sensitivity. It must be programmed together with the BKP polarity bit. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register)."]
pub type BKCMP4P_R = crate::BitReader;
#[doc = "Field `BKCMP4P` writer - tim_brk_cmp4 input polarity This bit selects the tim_brk_cmp4 input sensitivity. It must be programmed together with the BKP polarity bit. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register)."]
pub type BKCMP4P_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ETRSEL` reader - etr_in source selection These bits select the etr_in input source. ... Refer to for product specific implementation. Note: These bits can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register)."]
pub type ETRSEL_R = crate::FieldReader;
#[doc = "Field `ETRSEL` writer - etr_in source selection These bits select the etr_in input source. ... Refer to for product specific implementation. Note: These bits can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register)."]
pub type ETRSEL_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bit 0 - TIMx_BKIN input enable This bit enables the TIMx_BKIN alternate function input for the timerâ\u{80}\u{99}s tim_brk input. TIMx_BKIN input is 'ORedâ\u{80}\u{99} with the other tim_brk sources. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register)."]
    #[inline(always)]
    pub fn bkine(&self) -> BKINE_R {
        BKINE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - tim_brk_cmp1 enable This bit enables the tim_brk_cmp1 for the timerâ\u{80}\u{99}s tim_brk input. tim_brk_cmp1 output is 'ORedâ\u{80}\u{99} with the other tim_brk sources. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register)."]
    #[inline(always)]
    pub fn bkcmp1e(&self) -> BKCMP1E_R {
        BKCMP1E_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - tim_brk_cmp2 enable This bit enables the tim_brk_cmp2 for the timerâ\u{80}\u{99}s tim_brk input. tim_brk_cmp2 output is 'ORedâ\u{80}\u{99} with the other tim_brk sources. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register)."]
    #[inline(always)]
    pub fn bkcmp2e(&self) -> BKCMP2E_R {
        BKCMP2E_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - tim_brk_cmp3 enable This bit enables the tim_brk_cmp3 for the timerâ\u{80}\u{99}s tim_brk input. tim_brk_cmp3 output is 'ORedâ\u{80}\u{99} with the other tim_brk sources. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register)."]
    #[inline(always)]
    pub fn bkcmp3e(&self) -> BKCMP3E_R {
        BKCMP3E_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - tim_brk_cmp4 enable This bit enables the tim_brk_cmp4 for the timerâ\u{80}\u{99}s tim_brk input. tim_brk_cmp4 output is 'ORedâ\u{80}\u{99} with the other tim_brk sources. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register)."]
    #[inline(always)]
    pub fn bkcmp4e(&self) -> BKCMP4E_R {
        BKCMP4E_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - tim_brk_cmp5 enable This bit enables the tim_brk_cmp5 for the timerâ\u{80}\u{99}s tim_brk input. tim_brk_cmp5 output is 'ORedâ\u{80}\u{99} with the other tim_brk sources. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register)."]
    #[inline(always)]
    pub fn bkcmp5e(&self) -> BKCMP5E_R {
        BKCMP5E_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - tim_brk_cmp6 enable This bit enables the tim_brk_cmp6 for the timerâ\u{80}\u{99}s tim_brk input. tim_brk_cmp6 output is 'ORedâ\u{80}\u{99} with the other tim_brk sources. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register)."]
    #[inline(always)]
    pub fn bkcmp6e(&self) -> BKCMP6E_R {
        BKCMP6E_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - tim_brk_cmp7 enable This bit enables the tim_brk_cmp7 for the timerâ\u{80}\u{99}s tim_brk input. tim_brk_cmp7 output is 'ORedâ\u{80}\u{99} with the other tim_brk sources. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register)."]
    #[inline(always)]
    pub fn bkcmp7e(&self) -> BKCMP7E_R {
        BKCMP7E_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - tim_brk_cmp8 enable This bit enables the tim_brk_cmp8 for the timerâ\u{80}\u{99}s tim_brk input. tim_brk_cmp8 output is 'ORedâ\u{80}\u{99} with the other tim_brk sources. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register)."]
    #[inline(always)]
    pub fn bkcmp8e(&self) -> BKCMP8E_R {
        BKCMP8E_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - TIMx_BKIN input polarity This bit selects the TIMx_BKIN alternate function input sensitivity. It must be programmed together with the BKP polarity bit. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register)."]
    #[inline(always)]
    pub fn bkinp(&self) -> BKINP_R {
        BKINP_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - tim_brk_cmp1 input polarity This bit selects the tim_brk_cmp1 input sensitivity. It must be programmed together with the BKP polarity bit. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register)."]
    #[inline(always)]
    pub fn bkcmp1p(&self) -> BKCMP1P_R {
        BKCMP1P_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - tim_brk_cmp2 input polarity This bit selects the tim_brk_cmp2 input sensitivity. It must be programmed together with the BKP polarity bit. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register)."]
    #[inline(always)]
    pub fn bkcmp2p(&self) -> BKCMP2P_R {
        BKCMP2P_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - tim_brk_cmp3 input polarity This bit selects the tim_brk_cmp3 input sensitivity. It must be programmed together with the BKP polarity bit. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register)."]
    #[inline(always)]
    pub fn bkcmp3p(&self) -> BKCMP3P_R {
        BKCMP3P_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - tim_brk_cmp4 input polarity This bit selects the tim_brk_cmp4 input sensitivity. It must be programmed together with the BKP polarity bit. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register)."]
    #[inline(always)]
    pub fn bkcmp4p(&self) -> BKCMP4P_R {
        BKCMP4P_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bits 14:17 - etr_in source selection These bits select the etr_in input source. ... Refer to for product specific implementation. Note: These bits can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register)."]
    #[inline(always)]
    pub fn etrsel(&self) -> ETRSEL_R {
        ETRSEL_R::new(((self.bits >> 14) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - TIMx_BKIN input enable This bit enables the TIMx_BKIN alternate function input for the timerâ\u{80}\u{99}s tim_brk input. TIMx_BKIN input is 'ORedâ\u{80}\u{99} with the other tim_brk sources. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register)."]
    #[inline(always)]
    #[must_use]
    pub fn bkine(&mut self) -> BKINE_W<TIM1_AF1rs> {
        BKINE_W::new(self, 0)
    }
    #[doc = "Bit 1 - tim_brk_cmp1 enable This bit enables the tim_brk_cmp1 for the timerâ\u{80}\u{99}s tim_brk input. tim_brk_cmp1 output is 'ORedâ\u{80}\u{99} with the other tim_brk sources. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register)."]
    #[inline(always)]
    #[must_use]
    pub fn bkcmp1e(&mut self) -> BKCMP1E_W<TIM1_AF1rs> {
        BKCMP1E_W::new(self, 1)
    }
    #[doc = "Bit 2 - tim_brk_cmp2 enable This bit enables the tim_brk_cmp2 for the timerâ\u{80}\u{99}s tim_brk input. tim_brk_cmp2 output is 'ORedâ\u{80}\u{99} with the other tim_brk sources. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register)."]
    #[inline(always)]
    #[must_use]
    pub fn bkcmp2e(&mut self) -> BKCMP2E_W<TIM1_AF1rs> {
        BKCMP2E_W::new(self, 2)
    }
    #[doc = "Bit 3 - tim_brk_cmp3 enable This bit enables the tim_brk_cmp3 for the timerâ\u{80}\u{99}s tim_brk input. tim_brk_cmp3 output is 'ORedâ\u{80}\u{99} with the other tim_brk sources. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register)."]
    #[inline(always)]
    #[must_use]
    pub fn bkcmp3e(&mut self) -> BKCMP3E_W<TIM1_AF1rs> {
        BKCMP3E_W::new(self, 3)
    }
    #[doc = "Bit 4 - tim_brk_cmp4 enable This bit enables the tim_brk_cmp4 for the timerâ\u{80}\u{99}s tim_brk input. tim_brk_cmp4 output is 'ORedâ\u{80}\u{99} with the other tim_brk sources. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register)."]
    #[inline(always)]
    #[must_use]
    pub fn bkcmp4e(&mut self) -> BKCMP4E_W<TIM1_AF1rs> {
        BKCMP4E_W::new(self, 4)
    }
    #[doc = "Bit 5 - tim_brk_cmp5 enable This bit enables the tim_brk_cmp5 for the timerâ\u{80}\u{99}s tim_brk input. tim_brk_cmp5 output is 'ORedâ\u{80}\u{99} with the other tim_brk sources. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register)."]
    #[inline(always)]
    #[must_use]
    pub fn bkcmp5e(&mut self) -> BKCMP5E_W<TIM1_AF1rs> {
        BKCMP5E_W::new(self, 5)
    }
    #[doc = "Bit 6 - tim_brk_cmp6 enable This bit enables the tim_brk_cmp6 for the timerâ\u{80}\u{99}s tim_brk input. tim_brk_cmp6 output is 'ORedâ\u{80}\u{99} with the other tim_brk sources. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register)."]
    #[inline(always)]
    #[must_use]
    pub fn bkcmp6e(&mut self) -> BKCMP6E_W<TIM1_AF1rs> {
        BKCMP6E_W::new(self, 6)
    }
    #[doc = "Bit 7 - tim_brk_cmp7 enable This bit enables the tim_brk_cmp7 for the timerâ\u{80}\u{99}s tim_brk input. tim_brk_cmp7 output is 'ORedâ\u{80}\u{99} with the other tim_brk sources. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register)."]
    #[inline(always)]
    #[must_use]
    pub fn bkcmp7e(&mut self) -> BKCMP7E_W<TIM1_AF1rs> {
        BKCMP7E_W::new(self, 7)
    }
    #[doc = "Bit 8 - tim_brk_cmp8 enable This bit enables the tim_brk_cmp8 for the timerâ\u{80}\u{99}s tim_brk input. tim_brk_cmp8 output is 'ORedâ\u{80}\u{99} with the other tim_brk sources. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register)."]
    #[inline(always)]
    #[must_use]
    pub fn bkcmp8e(&mut self) -> BKCMP8E_W<TIM1_AF1rs> {
        BKCMP8E_W::new(self, 8)
    }
    #[doc = "Bit 9 - TIMx_BKIN input polarity This bit selects the TIMx_BKIN alternate function input sensitivity. It must be programmed together with the BKP polarity bit. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register)."]
    #[inline(always)]
    #[must_use]
    pub fn bkinp(&mut self) -> BKINP_W<TIM1_AF1rs> {
        BKINP_W::new(self, 9)
    }
    #[doc = "Bit 10 - tim_brk_cmp1 input polarity This bit selects the tim_brk_cmp1 input sensitivity. It must be programmed together with the BKP polarity bit. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register)."]
    #[inline(always)]
    #[must_use]
    pub fn bkcmp1p(&mut self) -> BKCMP1P_W<TIM1_AF1rs> {
        BKCMP1P_W::new(self, 10)
    }
    #[doc = "Bit 11 - tim_brk_cmp2 input polarity This bit selects the tim_brk_cmp2 input sensitivity. It must be programmed together with the BKP polarity bit. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register)."]
    #[inline(always)]
    #[must_use]
    pub fn bkcmp2p(&mut self) -> BKCMP2P_W<TIM1_AF1rs> {
        BKCMP2P_W::new(self, 11)
    }
    #[doc = "Bit 12 - tim_brk_cmp3 input polarity This bit selects the tim_brk_cmp3 input sensitivity. It must be programmed together with the BKP polarity bit. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register)."]
    #[inline(always)]
    #[must_use]
    pub fn bkcmp3p(&mut self) -> BKCMP3P_W<TIM1_AF1rs> {
        BKCMP3P_W::new(self, 12)
    }
    #[doc = "Bit 13 - tim_brk_cmp4 input polarity This bit selects the tim_brk_cmp4 input sensitivity. It must be programmed together with the BKP polarity bit. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register)."]
    #[inline(always)]
    #[must_use]
    pub fn bkcmp4p(&mut self) -> BKCMP4P_W<TIM1_AF1rs> {
        BKCMP4P_W::new(self, 13)
    }
    #[doc = "Bits 14:17 - etr_in source selection These bits select the etr_in input source. ... Refer to for product specific implementation. Note: These bits can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register)."]
    #[inline(always)]
    #[must_use]
    pub fn etrsel(&mut self) -> ETRSEL_W<TIM1_AF1rs> {
        ETRSEL_W::new(self, 14)
    }
}
#[doc = "TIM1 alternate function option register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tim1_af1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tim1_af1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TIM1_AF1rs;
impl crate::RegisterSpec for TIM1_AF1rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tim1_af1::R`](R) reader structure"]
impl crate::Readable for TIM1_AF1rs {}
#[doc = "`write(|w| ..)` method takes [`tim1_af1::W`](W) writer structure"]
impl crate::Writable for TIM1_AF1rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TIM1_AF1 to value 0x01"]
impl crate::Resettable for TIM1_AF1rs {
    const RESET_VALUE: u32 = 0x01;
}
