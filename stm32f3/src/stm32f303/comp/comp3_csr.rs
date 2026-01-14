///Register `COMP3_CSR` reader
pub type R = crate::R<COMP3_CSRrs>;
///Register `COMP3_CSR` writer
pub type W = crate::W<COMP3_CSRrs>;
///Field `COMP3EN` reader - Comparator 3 enable
pub type COMP3EN_R = crate::BitReader;
///Field `COMP3EN` writer - Comparator 3 enable
pub type COMP3EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `COMP3MODE` reader - Comparator 3 mode
pub type COMP3MODE_R = crate::FieldReader;
///Field `COMP3MODE` writer - Comparator 3 mode
pub type COMP3MODE_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `COMP3INMSEL` reader - Comparator 3 inverting input selection
pub type COMP3INMSEL_R = crate::FieldReader;
///Field `COMP3INMSEL` writer - Comparator 3 inverting input selection
pub type COMP3INMSEL_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `COMP3INPSEL` reader - Comparator 3 non inverted input
pub type COMP3INPSEL_R = crate::BitReader;
///Field `COMP3INPSEL` writer - Comparator 3 non inverted input
pub type COMP3INPSEL_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `COMP3OUTSEL` reader - Comparator 3 output selection
pub type COMP3OUTSEL_R = crate::FieldReader;
///Field `COMP3OUTSEL` writer - Comparator 3 output selection
pub type COMP3OUTSEL_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `COMP3POL` reader - Comparator 3 output polarity
pub type COMP3POL_R = crate::BitReader;
///Field `COMP3POL` writer - Comparator 3 output polarity
pub type COMP3POL_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `COMP3HYST` reader - Comparator 3 hysteresis
pub type COMP3HYST_R = crate::FieldReader;
///Field `COMP3HYST` writer - Comparator 3 hysteresis
pub type COMP3HYST_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `COMP3_BLANKING` reader - Comparator 3 blanking source
pub type COMP3_BLANKING_R = crate::FieldReader;
///Field `COMP3_BLANKING` writer - Comparator 3 blanking source
pub type COMP3_BLANKING_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `COMP3OUT` reader - Comparator 3 output
pub type COMP3OUT_R = crate::BitReader;
///Field `COMP3LOCK` reader - Comparator 3 lock
pub type COMP3LOCK_R = crate::BitReader;
///Field `COMP3LOCK` writer - Comparator 3 lock
pub type COMP3LOCK_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - Comparator 3 enable
    #[inline(always)]
    pub fn comp3en(&self) -> COMP3EN_R {
        COMP3EN_R::new((self.bits & 1) != 0)
    }
    ///Bits 2:3 - Comparator 3 mode
    #[inline(always)]
    pub fn comp3mode(&self) -> COMP3MODE_R {
        COMP3MODE_R::new(((self.bits >> 2) & 3) as u8)
    }
    ///Bits 4:6 - Comparator 3 inverting input selection
    #[inline(always)]
    pub fn comp3inmsel(&self) -> COMP3INMSEL_R {
        COMP3INMSEL_R::new(((self.bits >> 4) & 7) as u8)
    }
    ///Bit 7 - Comparator 3 non inverted input
    #[inline(always)]
    pub fn comp3inpsel(&self) -> COMP3INPSEL_R {
        COMP3INPSEL_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bits 10:13 - Comparator 3 output selection
    #[inline(always)]
    pub fn comp3outsel(&self) -> COMP3OUTSEL_R {
        COMP3OUTSEL_R::new(((self.bits >> 10) & 0x0f) as u8)
    }
    ///Bit 15 - Comparator 3 output polarity
    #[inline(always)]
    pub fn comp3pol(&self) -> COMP3POL_R {
        COMP3POL_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bits 16:17 - Comparator 3 hysteresis
    #[inline(always)]
    pub fn comp3hyst(&self) -> COMP3HYST_R {
        COMP3HYST_R::new(((self.bits >> 16) & 3) as u8)
    }
    ///Bits 18:20 - Comparator 3 blanking source
    #[inline(always)]
    pub fn comp3_blanking(&self) -> COMP3_BLANKING_R {
        COMP3_BLANKING_R::new(((self.bits >> 18) & 7) as u8)
    }
    ///Bit 30 - Comparator 3 output
    #[inline(always)]
    pub fn comp3out(&self) -> COMP3OUT_R {
        COMP3OUT_R::new(((self.bits >> 30) & 1) != 0)
    }
    ///Bit 31 - Comparator 3 lock
    #[inline(always)]
    pub fn comp3lock(&self) -> COMP3LOCK_R {
        COMP3LOCK_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("COMP3_CSR")
            .field("comp3en", &self.comp3en())
            .field("comp3mode", &self.comp3mode())
            .field("comp3inmsel", &self.comp3inmsel())
            .field("comp3inpsel", &self.comp3inpsel())
            .field("comp3outsel", &self.comp3outsel())
            .field("comp3pol", &self.comp3pol())
            .field("comp3hyst", &self.comp3hyst())
            .field("comp3_blanking", &self.comp3_blanking())
            .field("comp3out", &self.comp3out())
            .field("comp3lock", &self.comp3lock())
            .finish()
    }
}
impl W {
    ///Bit 0 - Comparator 3 enable
    #[inline(always)]
    pub fn comp3en(&mut self) -> COMP3EN_W<'_, COMP3_CSRrs> {
        COMP3EN_W::new(self, 0)
    }
    ///Bits 2:3 - Comparator 3 mode
    #[inline(always)]
    pub fn comp3mode(&mut self) -> COMP3MODE_W<'_, COMP3_CSRrs> {
        COMP3MODE_W::new(self, 2)
    }
    ///Bits 4:6 - Comparator 3 inverting input selection
    #[inline(always)]
    pub fn comp3inmsel(&mut self) -> COMP3INMSEL_W<'_, COMP3_CSRrs> {
        COMP3INMSEL_W::new(self, 4)
    }
    ///Bit 7 - Comparator 3 non inverted input
    #[inline(always)]
    pub fn comp3inpsel(&mut self) -> COMP3INPSEL_W<'_, COMP3_CSRrs> {
        COMP3INPSEL_W::new(self, 7)
    }
    ///Bits 10:13 - Comparator 3 output selection
    #[inline(always)]
    pub fn comp3outsel(&mut self) -> COMP3OUTSEL_W<'_, COMP3_CSRrs> {
        COMP3OUTSEL_W::new(self, 10)
    }
    ///Bit 15 - Comparator 3 output polarity
    #[inline(always)]
    pub fn comp3pol(&mut self) -> COMP3POL_W<'_, COMP3_CSRrs> {
        COMP3POL_W::new(self, 15)
    }
    ///Bits 16:17 - Comparator 3 hysteresis
    #[inline(always)]
    pub fn comp3hyst(&mut self) -> COMP3HYST_W<'_, COMP3_CSRrs> {
        COMP3HYST_W::new(self, 16)
    }
    ///Bits 18:20 - Comparator 3 blanking source
    #[inline(always)]
    pub fn comp3_blanking(&mut self) -> COMP3_BLANKING_W<'_, COMP3_CSRrs> {
        COMP3_BLANKING_W::new(self, 18)
    }
    ///Bit 31 - Comparator 3 lock
    #[inline(always)]
    pub fn comp3lock(&mut self) -> COMP3LOCK_W<'_, COMP3_CSRrs> {
        COMP3LOCK_W::new(self, 31)
    }
}
/**control and status register

You can [`read`](crate::Reg::read) this register and get [`comp3_csr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`comp3_csr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F303.html#COMP:COMP3_CSR)*/
pub struct COMP3_CSRrs;
impl crate::RegisterSpec for COMP3_CSRrs {
    type Ux = u32;
}
///`read()` method returns [`comp3_csr::R`](R) reader structure
impl crate::Readable for COMP3_CSRrs {}
///`write(|w| ..)` method takes [`comp3_csr::W`](W) writer structure
impl crate::Writable for COMP3_CSRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets COMP3_CSR to value 0
impl crate::Resettable for COMP3_CSRrs {}
