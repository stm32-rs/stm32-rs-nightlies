///Register `COMP5_CSR` reader
pub type R = crate::R<COMP5_CSRrs>;
///Register `COMP5_CSR` writer
pub type W = crate::W<COMP5_CSRrs>;
///Field `COMP5EN` reader - Comparator 5 enable
pub type COMP5EN_R = crate::BitReader;
///Field `COMP5EN` writer - Comparator 5 enable
pub type COMP5EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `COMP5MODE` reader - Comparator 5 mode
pub type COMP5MODE_R = crate::FieldReader;
///Field `COMP5MODE` writer - Comparator 5 mode
pub type COMP5MODE_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `COMP5INMSEL` reader - Comparator 5 inverting input selection
pub type COMP5INMSEL_R = crate::FieldReader;
///Field `COMP5INMSEL` writer - Comparator 5 inverting input selection
pub type COMP5INMSEL_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `COMP5INPSEL` reader - Comparator 5 non inverted input
pub type COMP5INPSEL_R = crate::BitReader;
///Field `COMP5INPSEL` writer - Comparator 5 non inverted input
pub type COMP5INPSEL_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `COMP5OUTSEL` reader - Comparator 5 output selection
pub type COMP5OUTSEL_R = crate::FieldReader;
///Field `COMP5OUTSEL` writer - Comparator 5 output selection
pub type COMP5OUTSEL_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `COMP5POL` reader - Comparator 5 output polarity
pub type COMP5POL_R = crate::BitReader;
///Field `COMP5POL` writer - Comparator 5 output polarity
pub type COMP5POL_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `COMP5HYST` reader - Comparator 5 hysteresis
pub type COMP5HYST_R = crate::FieldReader;
///Field `COMP5HYST` writer - Comparator 5 hysteresis
pub type COMP5HYST_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `COMP5_BLANKING` reader - Comparator 5 blanking source
pub type COMP5_BLANKING_R = crate::FieldReader;
///Field `COMP5_BLANKING` writer - Comparator 5 blanking source
pub type COMP5_BLANKING_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `COMP5OUT` reader - Comparator 5 output
pub type COMP5OUT_R = crate::BitReader;
///Field `COMP5LOCK` reader - Comparator 5 lock
pub type COMP5LOCK_R = crate::BitReader;
///Field `COMP5LOCK` writer - Comparator 5 lock
pub type COMP5LOCK_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - Comparator 5 enable
    #[inline(always)]
    pub fn comp5en(&self) -> COMP5EN_R {
        COMP5EN_R::new((self.bits & 1) != 0)
    }
    ///Bits 2:3 - Comparator 5 mode
    #[inline(always)]
    pub fn comp5mode(&self) -> COMP5MODE_R {
        COMP5MODE_R::new(((self.bits >> 2) & 3) as u8)
    }
    ///Bits 4:6 - Comparator 5 inverting input selection
    #[inline(always)]
    pub fn comp5inmsel(&self) -> COMP5INMSEL_R {
        COMP5INMSEL_R::new(((self.bits >> 4) & 7) as u8)
    }
    ///Bit 7 - Comparator 5 non inverted input
    #[inline(always)]
    pub fn comp5inpsel(&self) -> COMP5INPSEL_R {
        COMP5INPSEL_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bits 10:13 - Comparator 5 output selection
    #[inline(always)]
    pub fn comp5outsel(&self) -> COMP5OUTSEL_R {
        COMP5OUTSEL_R::new(((self.bits >> 10) & 0x0f) as u8)
    }
    ///Bit 15 - Comparator 5 output polarity
    #[inline(always)]
    pub fn comp5pol(&self) -> COMP5POL_R {
        COMP5POL_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bits 16:17 - Comparator 5 hysteresis
    #[inline(always)]
    pub fn comp5hyst(&self) -> COMP5HYST_R {
        COMP5HYST_R::new(((self.bits >> 16) & 3) as u8)
    }
    ///Bits 18:20 - Comparator 5 blanking source
    #[inline(always)]
    pub fn comp5_blanking(&self) -> COMP5_BLANKING_R {
        COMP5_BLANKING_R::new(((self.bits >> 18) & 7) as u8)
    }
    ///Bit 30 - Comparator 5 output
    #[inline(always)]
    pub fn comp5out(&self) -> COMP5OUT_R {
        COMP5OUT_R::new(((self.bits >> 30) & 1) != 0)
    }
    ///Bit 31 - Comparator 5 lock
    #[inline(always)]
    pub fn comp5lock(&self) -> COMP5LOCK_R {
        COMP5LOCK_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("COMP5_CSR")
            .field("comp5en", &self.comp5en())
            .field("comp5mode", &self.comp5mode())
            .field("comp5inmsel", &self.comp5inmsel())
            .field("comp5inpsel", &self.comp5inpsel())
            .field("comp5outsel", &self.comp5outsel())
            .field("comp5pol", &self.comp5pol())
            .field("comp5hyst", &self.comp5hyst())
            .field("comp5_blanking", &self.comp5_blanking())
            .field("comp5out", &self.comp5out())
            .field("comp5lock", &self.comp5lock())
            .finish()
    }
}
impl W {
    ///Bit 0 - Comparator 5 enable
    #[inline(always)]
    pub fn comp5en(&mut self) -> COMP5EN_W<'_, COMP5_CSRrs> {
        COMP5EN_W::new(self, 0)
    }
    ///Bits 2:3 - Comparator 5 mode
    #[inline(always)]
    pub fn comp5mode(&mut self) -> COMP5MODE_W<'_, COMP5_CSRrs> {
        COMP5MODE_W::new(self, 2)
    }
    ///Bits 4:6 - Comparator 5 inverting input selection
    #[inline(always)]
    pub fn comp5inmsel(&mut self) -> COMP5INMSEL_W<'_, COMP5_CSRrs> {
        COMP5INMSEL_W::new(self, 4)
    }
    ///Bit 7 - Comparator 5 non inverted input
    #[inline(always)]
    pub fn comp5inpsel(&mut self) -> COMP5INPSEL_W<'_, COMP5_CSRrs> {
        COMP5INPSEL_W::new(self, 7)
    }
    ///Bits 10:13 - Comparator 5 output selection
    #[inline(always)]
    pub fn comp5outsel(&mut self) -> COMP5OUTSEL_W<'_, COMP5_CSRrs> {
        COMP5OUTSEL_W::new(self, 10)
    }
    ///Bit 15 - Comparator 5 output polarity
    #[inline(always)]
    pub fn comp5pol(&mut self) -> COMP5POL_W<'_, COMP5_CSRrs> {
        COMP5POL_W::new(self, 15)
    }
    ///Bits 16:17 - Comparator 5 hysteresis
    #[inline(always)]
    pub fn comp5hyst(&mut self) -> COMP5HYST_W<'_, COMP5_CSRrs> {
        COMP5HYST_W::new(self, 16)
    }
    ///Bits 18:20 - Comparator 5 blanking source
    #[inline(always)]
    pub fn comp5_blanking(&mut self) -> COMP5_BLANKING_W<'_, COMP5_CSRrs> {
        COMP5_BLANKING_W::new(self, 18)
    }
    ///Bit 31 - Comparator 5 lock
    #[inline(always)]
    pub fn comp5lock(&mut self) -> COMP5LOCK_W<'_, COMP5_CSRrs> {
        COMP5LOCK_W::new(self, 31)
    }
}
/**control and status register

You can [`read`](crate::Reg::read) this register and get [`comp5_csr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`comp5_csr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F303.html#COMP:COMP5_CSR)*/
pub struct COMP5_CSRrs;
impl crate::RegisterSpec for COMP5_CSRrs {
    type Ux = u32;
}
///`read()` method returns [`comp5_csr::R`](R) reader structure
impl crate::Readable for COMP5_CSRrs {}
///`write(|w| ..)` method takes [`comp5_csr::W`](W) writer structure
impl crate::Writable for COMP5_CSRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets COMP5_CSR to value 0
impl crate::Resettable for COMP5_CSRrs {}
