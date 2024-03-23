#[doc = "Register `CR1` reader"]
pub type R = crate::R<CR1rs>;
#[doc = "Register `CR1` writer"]
pub type W = crate::W<CR1rs>;
#[doc = "Field `UE` reader - USART enable"]
pub type UE_R = crate::BitReader;
#[doc = "Field `UE` writer - USART enable"]
pub type UE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UESM` reader - USART enable in Stop mode"]
pub type UESM_R = crate::BitReader;
#[doc = "Field `UESM` writer - USART enable in Stop mode"]
pub type UESM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RE` reader - Receiver enable"]
pub type RE_R = crate::BitReader;
#[doc = "Field `RE` writer - Receiver enable"]
pub type RE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TE` reader - Transmitter enable"]
pub type TE_R = crate::BitReader;
#[doc = "Field `TE` writer - Transmitter enable"]
pub type TE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IDLEIE` reader - IDLE interrupt enable"]
pub type IDLEIE_R = crate::BitReader;
#[doc = "Field `IDLEIE` writer - IDLE interrupt enable"]
pub type IDLEIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXNEIE` reader - RXNE interrupt enable"]
pub type RXNEIE_R = crate::BitReader;
#[doc = "Field `RXNEIE` writer - RXNE interrupt enable"]
pub type RXNEIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TCIE` reader - Transmission complete interrupt enable"]
pub type TCIE_R = crate::BitReader;
#[doc = "Field `TCIE` writer - Transmission complete interrupt enable"]
pub type TCIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXEIE` reader - interrupt enable"]
pub type TXEIE_R = crate::BitReader;
#[doc = "Field `TXEIE` writer - interrupt enable"]
pub type TXEIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PEIE` reader - PE interrupt enable"]
pub type PEIE_R = crate::BitReader;
#[doc = "Field `PEIE` writer - PE interrupt enable"]
pub type PEIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PS` reader - Parity selection"]
pub type PS_R = crate::BitReader;
#[doc = "Field `PS` writer - Parity selection"]
pub type PS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PCE` reader - Parity control enable"]
pub type PCE_R = crate::BitReader;
#[doc = "Field `PCE` writer - Parity control enable"]
pub type PCE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WAKE` reader - Receiver wakeup method"]
pub type WAKE_R = crate::BitReader;
#[doc = "Field `WAKE` writer - Receiver wakeup method"]
pub type WAKE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `M0` reader - Word length"]
pub type M0_R = crate::BitReader;
#[doc = "Field `M0` writer - Word length"]
pub type M0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MME` reader - Mute mode enable"]
pub type MME_R = crate::BitReader;
#[doc = "Field `MME` writer - Mute mode enable"]
pub type MME_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CMIE` reader - Character match interrupt enable"]
pub type CMIE_R = crate::BitReader;
#[doc = "Field `CMIE` writer - Character match interrupt enable"]
pub type CMIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OVER8` reader - Oversampling mode"]
pub type OVER8_R = crate::BitReader;
#[doc = "Field `OVER8` writer - Oversampling mode"]
pub type OVER8_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DEDT0` reader - DEDT0"]
pub type DEDT0_R = crate::BitReader;
#[doc = "Field `DEDT0` writer - DEDT0"]
pub type DEDT0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DEDT1` reader - DEDT1"]
pub type DEDT1_R = crate::BitReader;
#[doc = "Field `DEDT1` writer - DEDT1"]
pub type DEDT1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DEDT2` reader - DEDT2"]
pub type DEDT2_R = crate::BitReader;
#[doc = "Field `DEDT2` writer - DEDT2"]
pub type DEDT2_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DEDT3` reader - DEDT3"]
pub type DEDT3_R = crate::BitReader;
#[doc = "Field `DEDT3` writer - DEDT3"]
pub type DEDT3_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DEDT4` reader - Driver Enable de-assertion time"]
pub type DEDT4_R = crate::BitReader;
#[doc = "Field `DEDT4` writer - Driver Enable de-assertion time"]
pub type DEDT4_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DEAT0` reader - DEAT0"]
pub type DEAT0_R = crate::BitReader;
#[doc = "Field `DEAT0` writer - DEAT0"]
pub type DEAT0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DEAT1` reader - DEAT1"]
pub type DEAT1_R = crate::BitReader;
#[doc = "Field `DEAT1` writer - DEAT1"]
pub type DEAT1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DEAT2` reader - DEAT2"]
pub type DEAT2_R = crate::BitReader;
#[doc = "Field `DEAT2` writer - DEAT2"]
pub type DEAT2_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DEAT3` reader - DEAT3"]
pub type DEAT3_R = crate::BitReader;
#[doc = "Field `DEAT3` writer - DEAT3"]
pub type DEAT3_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DEAT4` reader - Driver Enable assertion time"]
pub type DEAT4_R = crate::BitReader;
#[doc = "Field `DEAT4` writer - Driver Enable assertion time"]
pub type DEAT4_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RTOIE` reader - Receiver timeout interrupt enable"]
pub type RTOIE_R = crate::BitReader;
#[doc = "Field `RTOIE` writer - Receiver timeout interrupt enable"]
pub type RTOIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EOBIE` reader - End of Block interrupt enable"]
pub type EOBIE_R = crate::BitReader;
#[doc = "Field `EOBIE` writer - End of Block interrupt enable"]
pub type EOBIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `M1` reader - M1"]
pub type M1_R = crate::BitReader;
#[doc = "Field `M1` writer - M1"]
pub type M1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FIFOEN` reader - FIFOEN"]
pub type FIFOEN_R = crate::BitReader;
#[doc = "Field `FIFOEN` writer - FIFOEN"]
pub type FIFOEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXFEIE` reader - TXFEIE"]
pub type TXFEIE_R = crate::BitReader;
#[doc = "Field `TXFEIE` writer - TXFEIE"]
pub type TXFEIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXFFIE` reader - RXFFIE"]
pub type RXFFIE_R = crate::BitReader;
#[doc = "Field `RXFFIE` writer - RXFFIE"]
pub type RXFFIE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - USART enable"]
    #[inline(always)]
    pub fn ue(&self) -> UE_R {
        UE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - USART enable in Stop mode"]
    #[inline(always)]
    pub fn uesm(&self) -> UESM_R {
        UESM_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Receiver enable"]
    #[inline(always)]
    pub fn re(&self) -> RE_R {
        RE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Transmitter enable"]
    #[inline(always)]
    pub fn te(&self) -> TE_R {
        TE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - IDLE interrupt enable"]
    #[inline(always)]
    pub fn idleie(&self) -> IDLEIE_R {
        IDLEIE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - RXNE interrupt enable"]
    #[inline(always)]
    pub fn rxneie(&self) -> RXNEIE_R {
        RXNEIE_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Transmission complete interrupt enable"]
    #[inline(always)]
    pub fn tcie(&self) -> TCIE_R {
        TCIE_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - interrupt enable"]
    #[inline(always)]
    pub fn txeie(&self) -> TXEIE_R {
        TXEIE_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - PE interrupt enable"]
    #[inline(always)]
    pub fn peie(&self) -> PEIE_R {
        PEIE_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Parity selection"]
    #[inline(always)]
    pub fn ps(&self) -> PS_R {
        PS_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Parity control enable"]
    #[inline(always)]
    pub fn pce(&self) -> PCE_R {
        PCE_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Receiver wakeup method"]
    #[inline(always)]
    pub fn wake(&self) -> WAKE_R {
        WAKE_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Word length"]
    #[inline(always)]
    pub fn m0(&self) -> M0_R {
        M0_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Mute mode enable"]
    #[inline(always)]
    pub fn mme(&self) -> MME_R {
        MME_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Character match interrupt enable"]
    #[inline(always)]
    pub fn cmie(&self) -> CMIE_R {
        CMIE_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Oversampling mode"]
    #[inline(always)]
    pub fn over8(&self) -> OVER8_R {
        OVER8_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - DEDT0"]
    #[inline(always)]
    pub fn dedt0(&self) -> DEDT0_R {
        DEDT0_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - DEDT1"]
    #[inline(always)]
    pub fn dedt1(&self) -> DEDT1_R {
        DEDT1_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - DEDT2"]
    #[inline(always)]
    pub fn dedt2(&self) -> DEDT2_R {
        DEDT2_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - DEDT3"]
    #[inline(always)]
    pub fn dedt3(&self) -> DEDT3_R {
        DEDT3_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Driver Enable de-assertion time"]
    #[inline(always)]
    pub fn dedt4(&self) -> DEDT4_R {
        DEDT4_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - DEAT0"]
    #[inline(always)]
    pub fn deat0(&self) -> DEAT0_R {
        DEAT0_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - DEAT1"]
    #[inline(always)]
    pub fn deat1(&self) -> DEAT1_R {
        DEAT1_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - DEAT2"]
    #[inline(always)]
    pub fn deat2(&self) -> DEAT2_R {
        DEAT2_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - DEAT3"]
    #[inline(always)]
    pub fn deat3(&self) -> DEAT3_R {
        DEAT3_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Driver Enable assertion time"]
    #[inline(always)]
    pub fn deat4(&self) -> DEAT4_R {
        DEAT4_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Receiver timeout interrupt enable"]
    #[inline(always)]
    pub fn rtoie(&self) -> RTOIE_R {
        RTOIE_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - End of Block interrupt enable"]
    #[inline(always)]
    pub fn eobie(&self) -> EOBIE_R {
        EOBIE_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - M1"]
    #[inline(always)]
    pub fn m1(&self) -> M1_R {
        M1_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - FIFOEN"]
    #[inline(always)]
    pub fn fifoen(&self) -> FIFOEN_R {
        FIFOEN_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - TXFEIE"]
    #[inline(always)]
    pub fn txfeie(&self) -> TXFEIE_R {
        TXFEIE_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - RXFFIE"]
    #[inline(always)]
    pub fn rxffie(&self) -> RXFFIE_R {
        RXFFIE_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - USART enable"]
    #[inline(always)]
    #[must_use]
    pub fn ue(&mut self) -> UE_W<CR1rs> {
        UE_W::new(self, 0)
    }
    #[doc = "Bit 1 - USART enable in Stop mode"]
    #[inline(always)]
    #[must_use]
    pub fn uesm(&mut self) -> UESM_W<CR1rs> {
        UESM_W::new(self, 1)
    }
    #[doc = "Bit 2 - Receiver enable"]
    #[inline(always)]
    #[must_use]
    pub fn re(&mut self) -> RE_W<CR1rs> {
        RE_W::new(self, 2)
    }
    #[doc = "Bit 3 - Transmitter enable"]
    #[inline(always)]
    #[must_use]
    pub fn te(&mut self) -> TE_W<CR1rs> {
        TE_W::new(self, 3)
    }
    #[doc = "Bit 4 - IDLE interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn idleie(&mut self) -> IDLEIE_W<CR1rs> {
        IDLEIE_W::new(self, 4)
    }
    #[doc = "Bit 5 - RXNE interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn rxneie(&mut self) -> RXNEIE_W<CR1rs> {
        RXNEIE_W::new(self, 5)
    }
    #[doc = "Bit 6 - Transmission complete interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn tcie(&mut self) -> TCIE_W<CR1rs> {
        TCIE_W::new(self, 6)
    }
    #[doc = "Bit 7 - interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn txeie(&mut self) -> TXEIE_W<CR1rs> {
        TXEIE_W::new(self, 7)
    }
    #[doc = "Bit 8 - PE interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn peie(&mut self) -> PEIE_W<CR1rs> {
        PEIE_W::new(self, 8)
    }
    #[doc = "Bit 9 - Parity selection"]
    #[inline(always)]
    #[must_use]
    pub fn ps(&mut self) -> PS_W<CR1rs> {
        PS_W::new(self, 9)
    }
    #[doc = "Bit 10 - Parity control enable"]
    #[inline(always)]
    #[must_use]
    pub fn pce(&mut self) -> PCE_W<CR1rs> {
        PCE_W::new(self, 10)
    }
    #[doc = "Bit 11 - Receiver wakeup method"]
    #[inline(always)]
    #[must_use]
    pub fn wake(&mut self) -> WAKE_W<CR1rs> {
        WAKE_W::new(self, 11)
    }
    #[doc = "Bit 12 - Word length"]
    #[inline(always)]
    #[must_use]
    pub fn m0(&mut self) -> M0_W<CR1rs> {
        M0_W::new(self, 12)
    }
    #[doc = "Bit 13 - Mute mode enable"]
    #[inline(always)]
    #[must_use]
    pub fn mme(&mut self) -> MME_W<CR1rs> {
        MME_W::new(self, 13)
    }
    #[doc = "Bit 14 - Character match interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn cmie(&mut self) -> CMIE_W<CR1rs> {
        CMIE_W::new(self, 14)
    }
    #[doc = "Bit 15 - Oversampling mode"]
    #[inline(always)]
    #[must_use]
    pub fn over8(&mut self) -> OVER8_W<CR1rs> {
        OVER8_W::new(self, 15)
    }
    #[doc = "Bit 16 - DEDT0"]
    #[inline(always)]
    #[must_use]
    pub fn dedt0(&mut self) -> DEDT0_W<CR1rs> {
        DEDT0_W::new(self, 16)
    }
    #[doc = "Bit 17 - DEDT1"]
    #[inline(always)]
    #[must_use]
    pub fn dedt1(&mut self) -> DEDT1_W<CR1rs> {
        DEDT1_W::new(self, 17)
    }
    #[doc = "Bit 18 - DEDT2"]
    #[inline(always)]
    #[must_use]
    pub fn dedt2(&mut self) -> DEDT2_W<CR1rs> {
        DEDT2_W::new(self, 18)
    }
    #[doc = "Bit 19 - DEDT3"]
    #[inline(always)]
    #[must_use]
    pub fn dedt3(&mut self) -> DEDT3_W<CR1rs> {
        DEDT3_W::new(self, 19)
    }
    #[doc = "Bit 20 - Driver Enable de-assertion time"]
    #[inline(always)]
    #[must_use]
    pub fn dedt4(&mut self) -> DEDT4_W<CR1rs> {
        DEDT4_W::new(self, 20)
    }
    #[doc = "Bit 21 - DEAT0"]
    #[inline(always)]
    #[must_use]
    pub fn deat0(&mut self) -> DEAT0_W<CR1rs> {
        DEAT0_W::new(self, 21)
    }
    #[doc = "Bit 22 - DEAT1"]
    #[inline(always)]
    #[must_use]
    pub fn deat1(&mut self) -> DEAT1_W<CR1rs> {
        DEAT1_W::new(self, 22)
    }
    #[doc = "Bit 23 - DEAT2"]
    #[inline(always)]
    #[must_use]
    pub fn deat2(&mut self) -> DEAT2_W<CR1rs> {
        DEAT2_W::new(self, 23)
    }
    #[doc = "Bit 24 - DEAT3"]
    #[inline(always)]
    #[must_use]
    pub fn deat3(&mut self) -> DEAT3_W<CR1rs> {
        DEAT3_W::new(self, 24)
    }
    #[doc = "Bit 25 - Driver Enable assertion time"]
    #[inline(always)]
    #[must_use]
    pub fn deat4(&mut self) -> DEAT4_W<CR1rs> {
        DEAT4_W::new(self, 25)
    }
    #[doc = "Bit 26 - Receiver timeout interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn rtoie(&mut self) -> RTOIE_W<CR1rs> {
        RTOIE_W::new(self, 26)
    }
    #[doc = "Bit 27 - End of Block interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn eobie(&mut self) -> EOBIE_W<CR1rs> {
        EOBIE_W::new(self, 27)
    }
    #[doc = "Bit 28 - M1"]
    #[inline(always)]
    #[must_use]
    pub fn m1(&mut self) -> M1_W<CR1rs> {
        M1_W::new(self, 28)
    }
    #[doc = "Bit 29 - FIFOEN"]
    #[inline(always)]
    #[must_use]
    pub fn fifoen(&mut self) -> FIFOEN_W<CR1rs> {
        FIFOEN_W::new(self, 29)
    }
    #[doc = "Bit 30 - TXFEIE"]
    #[inline(always)]
    #[must_use]
    pub fn txfeie(&mut self) -> TXFEIE_W<CR1rs> {
        TXFEIE_W::new(self, 30)
    }
    #[doc = "Bit 31 - RXFFIE"]
    #[inline(always)]
    #[must_use]
    pub fn rxffie(&mut self) -> RXFFIE_W<CR1rs> {
        RXFFIE_W::new(self, 31)
    }
}
#[doc = "Control register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cr1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cr1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CR1rs;
impl crate::RegisterSpec for CR1rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cr1::R`](R) reader structure"]
impl crate::Readable for CR1rs {}
#[doc = "`write(|w| ..)` method takes [`cr1::W`](W) writer structure"]
impl crate::Writable for CR1rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CR1 to value 0"]
impl crate::Resettable for CR1rs {
    const RESET_VALUE: u32 = 0;
}
