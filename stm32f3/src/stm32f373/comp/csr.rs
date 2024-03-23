#[doc = "Register `CSR` reader"]
pub type R = crate::R<CSRrs>;
#[doc = "Register `CSR` writer"]
pub type W = crate::W<CSRrs>;
#[doc = "Field `COMP1EN` reader - Comparator 1 enable"]
pub type COMP1EN_R = crate::BitReader;
#[doc = "Field `COMP1EN` writer - Comparator 1 enable"]
pub type COMP1EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `COMP1_INP_DAC` reader - Comparator 1 non inverting input connection to DAC output"]
pub type COMP1_INP_DAC_R = crate::BitReader;
#[doc = "Field `COMP1_INP_DAC` writer - Comparator 1 non inverting input connection to DAC output"]
pub type COMP1_INP_DAC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `COMP1MODE` reader - Comparator 1 mode"]
pub type COMP1MODE_R = crate::FieldReader;
#[doc = "Field `COMP1MODE` writer - Comparator 1 mode"]
pub type COMP1MODE_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `COMP1INSEL` reader - Comparator 1 inverting input selection"]
pub type COMP1INSEL_R = crate::FieldReader;
#[doc = "Field `COMP1INSEL` writer - Comparator 1 inverting input selection"]
pub type COMP1INSEL_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `COMP1OUTSEL` reader - Comparator 1 output selection"]
pub type COMP1OUTSEL_R = crate::FieldReader;
#[doc = "Field `COMP1OUTSEL` writer - Comparator 1 output selection"]
pub type COMP1OUTSEL_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `COMP1POL` reader - Comparator 1 output polarity"]
pub type COMP1POL_R = crate::BitReader;
#[doc = "Field `COMP1POL` writer - Comparator 1 output polarity"]
pub type COMP1POL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `COMP1HYST` reader - Comparator 1 hysteresis"]
pub type COMP1HYST_R = crate::FieldReader;
#[doc = "Field `COMP1HYST` writer - Comparator 1 hysteresis"]
pub type COMP1HYST_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `COMP1OUT` reader - Comparator 1 output"]
pub type COMP1OUT_R = crate::BitReader;
#[doc = "Field `COMP1LOCK` reader - Comparator 1 lock"]
pub type COMP1LOCK_R = crate::BitReader;
#[doc = "Field `COMP1LOCK` writer - Comparator 1 lock"]
pub type COMP1LOCK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `COMP2EN` reader - Comparator 2 enable"]
pub type COMP2EN_R = crate::BitReader;
#[doc = "Field `COMP2EN` writer - Comparator 2 enable"]
pub type COMP2EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `COMP2MODE` reader - Comparator 2 mode"]
pub type COMP2MODE_R = crate::FieldReader;
#[doc = "Field `COMP2MODE` writer - Comparator 2 mode"]
pub type COMP2MODE_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `COMP2INSEL` reader - Comparator 2 inverting input selection"]
pub type COMP2INSEL_R = crate::FieldReader;
#[doc = "Field `COMP2INSEL` writer - Comparator 2 inverting input selection"]
pub type COMP2INSEL_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `WNDWEN` reader - Window mode enable"]
pub type WNDWEN_R = crate::BitReader;
#[doc = "Field `WNDWEN` writer - Window mode enable"]
pub type WNDWEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `COMP2OUTSEL` reader - Comparator 2 output selection"]
pub type COMP2OUTSEL_R = crate::FieldReader;
#[doc = "Field `COMP2OUTSEL` writer - Comparator 2 output selection"]
pub type COMP2OUTSEL_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `COMP2POL` reader - Comparator 2 output polarity"]
pub type COMP2POL_R = crate::BitReader;
#[doc = "Field `COMP2POL` writer - Comparator 2 output polarity"]
pub type COMP2POL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `COMP2HYST` reader - Comparator 2 hysteresis"]
pub type COMP2HYST_R = crate::FieldReader;
#[doc = "Field `COMP2HYST` writer - Comparator 2 hysteresis"]
pub type COMP2HYST_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `COMP2OUT` reader - Comparator 2 output"]
pub type COMP2OUT_R = crate::BitReader;
#[doc = "Field `COMP2LOCK` reader - Comparator 2 lock"]
pub type COMP2LOCK_R = crate::BitReader;
#[doc = "Field `COMP2LOCK` writer - Comparator 2 lock"]
pub type COMP2LOCK_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Comparator 1 enable"]
    #[inline(always)]
    pub fn comp1en(&self) -> COMP1EN_R {
        COMP1EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Comparator 1 non inverting input connection to DAC output"]
    #[inline(always)]
    pub fn comp1_inp_dac(&self) -> COMP1_INP_DAC_R {
        COMP1_INP_DAC_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:3 - Comparator 1 mode"]
    #[inline(always)]
    pub fn comp1mode(&self) -> COMP1MODE_R {
        COMP1MODE_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:6 - Comparator 1 inverting input selection"]
    #[inline(always)]
    pub fn comp1insel(&self) -> COMP1INSEL_R {
        COMP1INSEL_R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bits 8:10 - Comparator 1 output selection"]
    #[inline(always)]
    pub fn comp1outsel(&self) -> COMP1OUTSEL_R {
        COMP1OUTSEL_R::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bit 11 - Comparator 1 output polarity"]
    #[inline(always)]
    pub fn comp1pol(&self) -> COMP1POL_R {
        COMP1POL_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 12:13 - Comparator 1 hysteresis"]
    #[inline(always)]
    pub fn comp1hyst(&self) -> COMP1HYST_R {
        COMP1HYST_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bit 14 - Comparator 1 output"]
    #[inline(always)]
    pub fn comp1out(&self) -> COMP1OUT_R {
        COMP1OUT_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Comparator 1 lock"]
    #[inline(always)]
    pub fn comp1lock(&self) -> COMP1LOCK_R {
        COMP1LOCK_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Comparator 2 enable"]
    #[inline(always)]
    pub fn comp2en(&self) -> COMP2EN_R {
        COMP2EN_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 18:19 - Comparator 2 mode"]
    #[inline(always)]
    pub fn comp2mode(&self) -> COMP2MODE_R {
        COMP2MODE_R::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bits 20:22 - Comparator 2 inverting input selection"]
    #[inline(always)]
    pub fn comp2insel(&self) -> COMP2INSEL_R {
        COMP2INSEL_R::new(((self.bits >> 20) & 7) as u8)
    }
    #[doc = "Bit 23 - Window mode enable"]
    #[inline(always)]
    pub fn wndwen(&self) -> WNDWEN_R {
        WNDWEN_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bits 24:26 - Comparator 2 output selection"]
    #[inline(always)]
    pub fn comp2outsel(&self) -> COMP2OUTSEL_R {
        COMP2OUTSEL_R::new(((self.bits >> 24) & 7) as u8)
    }
    #[doc = "Bit 27 - Comparator 2 output polarity"]
    #[inline(always)]
    pub fn comp2pol(&self) -> COMP2POL_R {
        COMP2POL_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bits 28:29 - Comparator 2 hysteresis"]
    #[inline(always)]
    pub fn comp2hyst(&self) -> COMP2HYST_R {
        COMP2HYST_R::new(((self.bits >> 28) & 3) as u8)
    }
    #[doc = "Bit 30 - Comparator 2 output"]
    #[inline(always)]
    pub fn comp2out(&self) -> COMP2OUT_R {
        COMP2OUT_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Comparator 2 lock"]
    #[inline(always)]
    pub fn comp2lock(&self) -> COMP2LOCK_R {
        COMP2LOCK_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Comparator 1 enable"]
    #[inline(always)]
    #[must_use]
    pub fn comp1en(&mut self) -> COMP1EN_W<CSRrs> {
        COMP1EN_W::new(self, 0)
    }
    #[doc = "Bit 1 - Comparator 1 non inverting input connection to DAC output"]
    #[inline(always)]
    #[must_use]
    pub fn comp1_inp_dac(&mut self) -> COMP1_INP_DAC_W<CSRrs> {
        COMP1_INP_DAC_W::new(self, 1)
    }
    #[doc = "Bits 2:3 - Comparator 1 mode"]
    #[inline(always)]
    #[must_use]
    pub fn comp1mode(&mut self) -> COMP1MODE_W<CSRrs> {
        COMP1MODE_W::new(self, 2)
    }
    #[doc = "Bits 4:6 - Comparator 1 inverting input selection"]
    #[inline(always)]
    #[must_use]
    pub fn comp1insel(&mut self) -> COMP1INSEL_W<CSRrs> {
        COMP1INSEL_W::new(self, 4)
    }
    #[doc = "Bits 8:10 - Comparator 1 output selection"]
    #[inline(always)]
    #[must_use]
    pub fn comp1outsel(&mut self) -> COMP1OUTSEL_W<CSRrs> {
        COMP1OUTSEL_W::new(self, 8)
    }
    #[doc = "Bit 11 - Comparator 1 output polarity"]
    #[inline(always)]
    #[must_use]
    pub fn comp1pol(&mut self) -> COMP1POL_W<CSRrs> {
        COMP1POL_W::new(self, 11)
    }
    #[doc = "Bits 12:13 - Comparator 1 hysteresis"]
    #[inline(always)]
    #[must_use]
    pub fn comp1hyst(&mut self) -> COMP1HYST_W<CSRrs> {
        COMP1HYST_W::new(self, 12)
    }
    #[doc = "Bit 15 - Comparator 1 lock"]
    #[inline(always)]
    #[must_use]
    pub fn comp1lock(&mut self) -> COMP1LOCK_W<CSRrs> {
        COMP1LOCK_W::new(self, 15)
    }
    #[doc = "Bit 16 - Comparator 2 enable"]
    #[inline(always)]
    #[must_use]
    pub fn comp2en(&mut self) -> COMP2EN_W<CSRrs> {
        COMP2EN_W::new(self, 16)
    }
    #[doc = "Bits 18:19 - Comparator 2 mode"]
    #[inline(always)]
    #[must_use]
    pub fn comp2mode(&mut self) -> COMP2MODE_W<CSRrs> {
        COMP2MODE_W::new(self, 18)
    }
    #[doc = "Bits 20:22 - Comparator 2 inverting input selection"]
    #[inline(always)]
    #[must_use]
    pub fn comp2insel(&mut self) -> COMP2INSEL_W<CSRrs> {
        COMP2INSEL_W::new(self, 20)
    }
    #[doc = "Bit 23 - Window mode enable"]
    #[inline(always)]
    #[must_use]
    pub fn wndwen(&mut self) -> WNDWEN_W<CSRrs> {
        WNDWEN_W::new(self, 23)
    }
    #[doc = "Bits 24:26 - Comparator 2 output selection"]
    #[inline(always)]
    #[must_use]
    pub fn comp2outsel(&mut self) -> COMP2OUTSEL_W<CSRrs> {
        COMP2OUTSEL_W::new(self, 24)
    }
    #[doc = "Bit 27 - Comparator 2 output polarity"]
    #[inline(always)]
    #[must_use]
    pub fn comp2pol(&mut self) -> COMP2POL_W<CSRrs> {
        COMP2POL_W::new(self, 27)
    }
    #[doc = "Bits 28:29 - Comparator 2 hysteresis"]
    #[inline(always)]
    #[must_use]
    pub fn comp2hyst(&mut self) -> COMP2HYST_W<CSRrs> {
        COMP2HYST_W::new(self, 28)
    }
    #[doc = "Bit 31 - Comparator 2 lock"]
    #[inline(always)]
    #[must_use]
    pub fn comp2lock(&mut self) -> COMP2LOCK_W<CSRrs> {
        COMP2LOCK_W::new(self, 31)
    }
}
#[doc = "control and status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`csr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`csr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CSRrs;
impl crate::RegisterSpec for CSRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`csr::R`](R) reader structure"]
impl crate::Readable for CSRrs {}
#[doc = "`write(|w| ..)` method takes [`csr::W`](W) writer structure"]
impl crate::Writable for CSRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CSR to value 0"]
impl crate::Resettable for CSRrs {
    const RESET_VALUE: u32 = 0;
}
