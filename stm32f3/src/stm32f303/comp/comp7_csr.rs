///Register `COMP7_CSR` reader
pub type R = crate::R<COMP7_CSRrs>;
///Register `COMP7_CSR` writer
pub type W = crate::W<COMP7_CSRrs>;
///Field `COMP7EN` reader - Comparator 7 enable
pub type COMP7EN_R = crate::BitReader;
///Field `COMP7EN` writer - Comparator 7 enable
pub type COMP7EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `COMP7MODE` reader - Comparator 7 mode
pub type COMP7MODE_R = crate::FieldReader;
///Field `COMP7MODE` writer - Comparator 7 mode
pub type COMP7MODE_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `COMP7INMSEL` reader - Comparator 7 inverting input selection
pub type COMP7INMSEL_R = crate::FieldReader;
///Field `COMP7INMSEL` writer - Comparator 7 inverting input selection
pub type COMP7INMSEL_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `COMP7INPSEL` reader - Comparator 7 non inverted input
pub type COMP7INPSEL_R = crate::BitReader;
///Field `COMP7INPSEL` writer - Comparator 7 non inverted input
pub type COMP7INPSEL_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `COMP7OUTSEL` reader - Comparator 7 output selection
pub type COMP7OUTSEL_R = crate::FieldReader;
///Field `COMP7OUTSEL` writer - Comparator 7 output selection
pub type COMP7OUTSEL_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `COMP7POL` reader - Comparator 7 output polarity
pub type COMP7POL_R = crate::BitReader;
///Field `COMP7POL` writer - Comparator 7 output polarity
pub type COMP7POL_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `COMP7HYST` reader - Comparator 7 hysteresis
pub type COMP7HYST_R = crate::FieldReader;
///Field `COMP7HYST` writer - Comparator 7 hysteresis
pub type COMP7HYST_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `COMP7_BLANKING` reader - Comparator 7 blanking source
pub type COMP7_BLANKING_R = crate::FieldReader;
///Field `COMP7_BLANKING` writer - Comparator 7 blanking source
pub type COMP7_BLANKING_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `COMP7OUT` reader - Comparator 7 output
pub type COMP7OUT_R = crate::BitReader;
///Field `COMP7LOCK` reader - Comparator 7 lock
pub type COMP7LOCK_R = crate::BitReader;
///Field `COMP7LOCK` writer - Comparator 7 lock
pub type COMP7LOCK_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - Comparator 7 enable
    #[inline(always)]
    pub fn comp7en(&self) -> COMP7EN_R {
        COMP7EN_R::new((self.bits & 1) != 0)
    }
    ///Bits 2:3 - Comparator 7 mode
    #[inline(always)]
    pub fn comp7mode(&self) -> COMP7MODE_R {
        COMP7MODE_R::new(((self.bits >> 2) & 3) as u8)
    }
    ///Bits 4:6 - Comparator 7 inverting input selection
    #[inline(always)]
    pub fn comp7inmsel(&self) -> COMP7INMSEL_R {
        COMP7INMSEL_R::new(((self.bits >> 4) & 7) as u8)
    }
    ///Bit 7 - Comparator 7 non inverted input
    #[inline(always)]
    pub fn comp7inpsel(&self) -> COMP7INPSEL_R {
        COMP7INPSEL_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bits 10:13 - Comparator 7 output selection
    #[inline(always)]
    pub fn comp7outsel(&self) -> COMP7OUTSEL_R {
        COMP7OUTSEL_R::new(((self.bits >> 10) & 0x0f) as u8)
    }
    ///Bit 15 - Comparator 7 output polarity
    #[inline(always)]
    pub fn comp7pol(&self) -> COMP7POL_R {
        COMP7POL_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bits 16:17 - Comparator 7 hysteresis
    #[inline(always)]
    pub fn comp7hyst(&self) -> COMP7HYST_R {
        COMP7HYST_R::new(((self.bits >> 16) & 3) as u8)
    }
    ///Bits 18:20 - Comparator 7 blanking source
    #[inline(always)]
    pub fn comp7_blanking(&self) -> COMP7_BLANKING_R {
        COMP7_BLANKING_R::new(((self.bits >> 18) & 7) as u8)
    }
    ///Bit 30 - Comparator 7 output
    #[inline(always)]
    pub fn comp7out(&self) -> COMP7OUT_R {
        COMP7OUT_R::new(((self.bits >> 30) & 1) != 0)
    }
    ///Bit 31 - Comparator 7 lock
    #[inline(always)]
    pub fn comp7lock(&self) -> COMP7LOCK_R {
        COMP7LOCK_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("COMP7_CSR")
            .field("comp7en", &self.comp7en())
            .field("comp7mode", &self.comp7mode())
            .field("comp7inmsel", &self.comp7inmsel())
            .field("comp7inpsel", &self.comp7inpsel())
            .field("comp7outsel", &self.comp7outsel())
            .field("comp7pol", &self.comp7pol())
            .field("comp7hyst", &self.comp7hyst())
            .field("comp7_blanking", &self.comp7_blanking())
            .field("comp7out", &self.comp7out())
            .field("comp7lock", &self.comp7lock())
            .finish()
    }
}
impl W {
    ///Bit 0 - Comparator 7 enable
    #[inline(always)]
    pub fn comp7en(&mut self) -> COMP7EN_W<COMP7_CSRrs> {
        COMP7EN_W::new(self, 0)
    }
    ///Bits 2:3 - Comparator 7 mode
    #[inline(always)]
    pub fn comp7mode(&mut self) -> COMP7MODE_W<COMP7_CSRrs> {
        COMP7MODE_W::new(self, 2)
    }
    ///Bits 4:6 - Comparator 7 inverting input selection
    #[inline(always)]
    pub fn comp7inmsel(&mut self) -> COMP7INMSEL_W<COMP7_CSRrs> {
        COMP7INMSEL_W::new(self, 4)
    }
    ///Bit 7 - Comparator 7 non inverted input
    #[inline(always)]
    pub fn comp7inpsel(&mut self) -> COMP7INPSEL_W<COMP7_CSRrs> {
        COMP7INPSEL_W::new(self, 7)
    }
    ///Bits 10:13 - Comparator 7 output selection
    #[inline(always)]
    pub fn comp7outsel(&mut self) -> COMP7OUTSEL_W<COMP7_CSRrs> {
        COMP7OUTSEL_W::new(self, 10)
    }
    ///Bit 15 - Comparator 7 output polarity
    #[inline(always)]
    pub fn comp7pol(&mut self) -> COMP7POL_W<COMP7_CSRrs> {
        COMP7POL_W::new(self, 15)
    }
    ///Bits 16:17 - Comparator 7 hysteresis
    #[inline(always)]
    pub fn comp7hyst(&mut self) -> COMP7HYST_W<COMP7_CSRrs> {
        COMP7HYST_W::new(self, 16)
    }
    ///Bits 18:20 - Comparator 7 blanking source
    #[inline(always)]
    pub fn comp7_blanking(&mut self) -> COMP7_BLANKING_W<COMP7_CSRrs> {
        COMP7_BLANKING_W::new(self, 18)
    }
    ///Bit 31 - Comparator 7 lock
    #[inline(always)]
    pub fn comp7lock(&mut self) -> COMP7LOCK_W<COMP7_CSRrs> {
        COMP7LOCK_W::new(self, 31)
    }
}
/**control and status register

You can [`read`](crate::Reg::read) this register and get [`comp7_csr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`comp7_csr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F303.html#COMP:COMP7_CSR)*/
pub struct COMP7_CSRrs;
impl crate::RegisterSpec for COMP7_CSRrs {
    type Ux = u32;
}
///`read()` method returns [`comp7_csr::R`](R) reader structure
impl crate::Readable for COMP7_CSRrs {}
///`write(|w| ..)` method takes [`comp7_csr::W`](W) writer structure
impl crate::Writable for COMP7_CSRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets COMP7_CSR to value 0
impl crate::Resettable for COMP7_CSRrs {}
