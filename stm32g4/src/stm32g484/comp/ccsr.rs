///Register `C%sCSR` reader
pub type R = crate::R<CCSRrs>;
///Register `C%sCSR` writer
pub type W = crate::W<CCSRrs>;
///Field `EN` reader - EN
pub type EN_R = crate::BitReader;
///Field `EN` writer - EN
pub type EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `INMSEL` reader - INMSEL
pub type INMSEL_R = crate::FieldReader;
///Field `INMSEL` writer - INMSEL
pub type INMSEL_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `INPSEL` reader - INPSEL
pub type INPSEL_R = crate::BitReader;
///Field `INPSEL` writer - INPSEL
pub type INPSEL_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `POL` reader - POL
pub type POL_R = crate::BitReader;
///Field `POL` writer - POL
pub type POL_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `HYST` reader - HYST
pub type HYST_R = crate::FieldReader;
///Field `HYST` writer - HYST
pub type HYST_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `BLANKSEL` reader - BLANKSEL
pub type BLANKSEL_R = crate::FieldReader;
///Field `BLANKSEL` writer - BLANKSEL
pub type BLANKSEL_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `BRGEN` reader - BRGEN
pub type BRGEN_R = crate::BitReader;
///Field `BRGEN` writer - BRGEN
pub type BRGEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SCALEN` reader - SCALEN
pub type SCALEN_R = crate::BitReader;
///Field `SCALEN` writer - SCALEN
pub type SCALEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `VALUE` reader - VALUE
pub type VALUE_R = crate::BitReader;
///Field `LOCK` reader - LOCK
pub type LOCK_R = crate::BitReader;
///Field `LOCK` writer - LOCK
pub type LOCK_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - EN
    #[inline(always)]
    pub fn en(&self) -> EN_R {
        EN_R::new((self.bits & 1) != 0)
    }
    ///Bits 4:6 - INMSEL
    #[inline(always)]
    pub fn inmsel(&self) -> INMSEL_R {
        INMSEL_R::new(((self.bits >> 4) & 7) as u8)
    }
    ///Bit 8 - INPSEL
    #[inline(always)]
    pub fn inpsel(&self) -> INPSEL_R {
        INPSEL_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 15 - POL
    #[inline(always)]
    pub fn pol(&self) -> POL_R {
        POL_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bits 16:18 - HYST
    #[inline(always)]
    pub fn hyst(&self) -> HYST_R {
        HYST_R::new(((self.bits >> 16) & 7) as u8)
    }
    ///Bits 19:21 - BLANKSEL
    #[inline(always)]
    pub fn blanksel(&self) -> BLANKSEL_R {
        BLANKSEL_R::new(((self.bits >> 19) & 7) as u8)
    }
    ///Bit 22 - BRGEN
    #[inline(always)]
    pub fn brgen(&self) -> BRGEN_R {
        BRGEN_R::new(((self.bits >> 22) & 1) != 0)
    }
    ///Bit 23 - SCALEN
    #[inline(always)]
    pub fn scalen(&self) -> SCALEN_R {
        SCALEN_R::new(((self.bits >> 23) & 1) != 0)
    }
    ///Bit 30 - VALUE
    #[inline(always)]
    pub fn value(&self) -> VALUE_R {
        VALUE_R::new(((self.bits >> 30) & 1) != 0)
    }
    ///Bit 31 - LOCK
    #[inline(always)]
    pub fn lock(&self) -> LOCK_R {
        LOCK_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CCSR")
            .field("en", &self.en())
            .field("inmsel", &self.inmsel())
            .field("inpsel", &self.inpsel())
            .field("pol", &self.pol())
            .field("hyst", &self.hyst())
            .field("blanksel", &self.blanksel())
            .field("brgen", &self.brgen())
            .field("scalen", &self.scalen())
            .field("value", &self.value())
            .field("lock", &self.lock())
            .finish()
    }
}
impl W {
    ///Bit 0 - EN
    #[inline(always)]
    pub fn en(&mut self) -> EN_W<'_, CCSRrs> {
        EN_W::new(self, 0)
    }
    ///Bits 4:6 - INMSEL
    #[inline(always)]
    pub fn inmsel(&mut self) -> INMSEL_W<'_, CCSRrs> {
        INMSEL_W::new(self, 4)
    }
    ///Bit 8 - INPSEL
    #[inline(always)]
    pub fn inpsel(&mut self) -> INPSEL_W<'_, CCSRrs> {
        INPSEL_W::new(self, 8)
    }
    ///Bit 15 - POL
    #[inline(always)]
    pub fn pol(&mut self) -> POL_W<'_, CCSRrs> {
        POL_W::new(self, 15)
    }
    ///Bits 16:18 - HYST
    #[inline(always)]
    pub fn hyst(&mut self) -> HYST_W<'_, CCSRrs> {
        HYST_W::new(self, 16)
    }
    ///Bits 19:21 - BLANKSEL
    #[inline(always)]
    pub fn blanksel(&mut self) -> BLANKSEL_W<'_, CCSRrs> {
        BLANKSEL_W::new(self, 19)
    }
    ///Bit 22 - BRGEN
    #[inline(always)]
    pub fn brgen(&mut self) -> BRGEN_W<'_, CCSRrs> {
        BRGEN_W::new(self, 22)
    }
    ///Bit 23 - SCALEN
    #[inline(always)]
    pub fn scalen(&mut self) -> SCALEN_W<'_, CCSRrs> {
        SCALEN_W::new(self, 23)
    }
    ///Bit 31 - LOCK
    #[inline(always)]
    pub fn lock(&mut self) -> LOCK_W<'_, CCSRrs> {
        LOCK_W::new(self, 31)
    }
}
/**Comparator control/status register

You can [`read`](crate::Reg::read) this register and get [`ccsr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccsr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32G484.html#COMP:C[1]CSR)*/
pub struct CCSRrs;
impl crate::RegisterSpec for CCSRrs {
    type Ux = u32;
}
///`read()` method returns [`ccsr::R`](R) reader structure
impl crate::Readable for CCSRrs {}
///`write(|w| ..)` method takes [`ccsr::W`](W) writer structure
impl crate::Writable for CCSRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets C%sCSR to value 0
impl crate::Resettable for CCSRrs {}
