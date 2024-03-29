#[doc = "Register `TIM1_DTR2` reader"]
pub type R = crate::R<TIM1_DTR2rs>;
#[doc = "Register `TIM1_DTR2` writer"]
pub type W = crate::W<TIM1_DTR2rs>;
#[doc = "Field `DTGF` reader - Dead-time falling edge generator setup This bit-field defines the duration of the dead-time inserted between the complementary outputs, on the falling edge. DTGF\\[7:5\\]=0xx => DTF=DTGF\\[7:0\\]x tdtg with tdtg=tDTS. DTGF\\[7:5\\]=10x => DTF=(64+DTGF\\[5:0\\])xtdtg with Tdtg=2xtDTS. DTGF\\[7:5\\]=110 => DTF=(32+DTGF\\[4:0\\])xtdtg with Tdtg=8xtDTS. DTGF\\[7:5\\]=111 => DTF=(32+DTGF\\[4:0\\])xtdtg with Tdtg=16xtDTS. Example if TDTS=125ns (8MHz), dead-time possible values are: 0 to 15875 ns by 125 ns steps, 16 us to 31750 ns by 250 ns steps, 32 us to 63us by 1 us steps, 64 us to 126 us by 2 us steps Note: This bit-field can not be modified as long as LOCK level 1, 2 or 3 has been programmed (LOCK bits in TIMx_BDTR register)."]
pub type DTGF_R = crate::FieldReader;
#[doc = "Field `DTGF` writer - Dead-time falling edge generator setup This bit-field defines the duration of the dead-time inserted between the complementary outputs, on the falling edge. DTGF\\[7:5\\]=0xx => DTF=DTGF\\[7:0\\]x tdtg with tdtg=tDTS. DTGF\\[7:5\\]=10x => DTF=(64+DTGF\\[5:0\\])xtdtg with Tdtg=2xtDTS. DTGF\\[7:5\\]=110 => DTF=(32+DTGF\\[4:0\\])xtdtg with Tdtg=8xtDTS. DTGF\\[7:5\\]=111 => DTF=(32+DTGF\\[4:0\\])xtdtg with Tdtg=16xtDTS. Example if TDTS=125ns (8MHz), dead-time possible values are: 0 to 15875 ns by 125 ns steps, 16 us to 31750 ns by 250 ns steps, 32 us to 63us by 1 us steps, 64 us to 126 us by 2 us steps Note: This bit-field can not be modified as long as LOCK level 1, 2 or 3 has been programmed (LOCK bits in TIMx_BDTR register)."]
pub type DTGF_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `DTAE` reader - Deadtime asymmetric enable Note: This bit can not be modified as long as LOCK level 1, 2 or 3 has been programmed (LOCK bits in TIMx_BDTR register)."]
pub type DTAE_R = crate::BitReader;
#[doc = "Field `DTAE` writer - Deadtime asymmetric enable Note: This bit can not be modified as long as LOCK level 1, 2 or 3 has been programmed (LOCK bits in TIMx_BDTR register)."]
pub type DTAE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DTPE` reader - Deadtime preload enable Note: This bit can not be modified as long as LOCK level 1, 2 or 3 has been programmed (LOCK bits in TIMx_BDTR register)."]
pub type DTPE_R = crate::BitReader;
#[doc = "Field `DTPE` writer - Deadtime preload enable Note: This bit can not be modified as long as LOCK level 1, 2 or 3 has been programmed (LOCK bits in TIMx_BDTR register)."]
pub type DTPE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:7 - Dead-time falling edge generator setup This bit-field defines the duration of the dead-time inserted between the complementary outputs, on the falling edge. DTGF\\[7:5\\]=0xx => DTF=DTGF\\[7:0\\]x tdtg with tdtg=tDTS. DTGF\\[7:5\\]=10x => DTF=(64+DTGF\\[5:0\\])xtdtg with Tdtg=2xtDTS. DTGF\\[7:5\\]=110 => DTF=(32+DTGF\\[4:0\\])xtdtg with Tdtg=8xtDTS. DTGF\\[7:5\\]=111 => DTF=(32+DTGF\\[4:0\\])xtdtg with Tdtg=16xtDTS. Example if TDTS=125ns (8MHz), dead-time possible values are: 0 to 15875 ns by 125 ns steps, 16 us to 31750 ns by 250 ns steps, 32 us to 63us by 1 us steps, 64 us to 126 us by 2 us steps Note: This bit-field can not be modified as long as LOCK level 1, 2 or 3 has been programmed (LOCK bits in TIMx_BDTR register)."]
    #[inline(always)]
    pub fn dtgf(&self) -> DTGF_R {
        DTGF_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 16 - Deadtime asymmetric enable Note: This bit can not be modified as long as LOCK level 1, 2 or 3 has been programmed (LOCK bits in TIMx_BDTR register)."]
    #[inline(always)]
    pub fn dtae(&self) -> DTAE_R {
        DTAE_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Deadtime preload enable Note: This bit can not be modified as long as LOCK level 1, 2 or 3 has been programmed (LOCK bits in TIMx_BDTR register)."]
    #[inline(always)]
    pub fn dtpe(&self) -> DTPE_R {
        DTPE_R::new(((self.bits >> 17) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - Dead-time falling edge generator setup This bit-field defines the duration of the dead-time inserted between the complementary outputs, on the falling edge. DTGF\\[7:5\\]=0xx => DTF=DTGF\\[7:0\\]x tdtg with tdtg=tDTS. DTGF\\[7:5\\]=10x => DTF=(64+DTGF\\[5:0\\])xtdtg with Tdtg=2xtDTS. DTGF\\[7:5\\]=110 => DTF=(32+DTGF\\[4:0\\])xtdtg with Tdtg=8xtDTS. DTGF\\[7:5\\]=111 => DTF=(32+DTGF\\[4:0\\])xtdtg with Tdtg=16xtDTS. Example if TDTS=125ns (8MHz), dead-time possible values are: 0 to 15875 ns by 125 ns steps, 16 us to 31750 ns by 250 ns steps, 32 us to 63us by 1 us steps, 64 us to 126 us by 2 us steps Note: This bit-field can not be modified as long as LOCK level 1, 2 or 3 has been programmed (LOCK bits in TIMx_BDTR register)."]
    #[inline(always)]
    #[must_use]
    pub fn dtgf(&mut self) -> DTGF_W<TIM1_DTR2rs> {
        DTGF_W::new(self, 0)
    }
    #[doc = "Bit 16 - Deadtime asymmetric enable Note: This bit can not be modified as long as LOCK level 1, 2 or 3 has been programmed (LOCK bits in TIMx_BDTR register)."]
    #[inline(always)]
    #[must_use]
    pub fn dtae(&mut self) -> DTAE_W<TIM1_DTR2rs> {
        DTAE_W::new(self, 16)
    }
    #[doc = "Bit 17 - Deadtime preload enable Note: This bit can not be modified as long as LOCK level 1, 2 or 3 has been programmed (LOCK bits in TIMx_BDTR register)."]
    #[inline(always)]
    #[must_use]
    pub fn dtpe(&mut self) -> DTPE_W<TIM1_DTR2rs> {
        DTPE_W::new(self, 17)
    }
}
#[doc = "TIM1 timer deadtime register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tim1_dtr2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tim1_dtr2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TIM1_DTR2rs;
impl crate::RegisterSpec for TIM1_DTR2rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tim1_dtr2::R`](R) reader structure"]
impl crate::Readable for TIM1_DTR2rs {}
#[doc = "`write(|w| ..)` method takes [`tim1_dtr2::W`](W) writer structure"]
impl crate::Writable for TIM1_DTR2rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TIM1_DTR2 to value 0"]
impl crate::Resettable for TIM1_DTR2rs {
    const RESET_VALUE: u32 = 0;
}
