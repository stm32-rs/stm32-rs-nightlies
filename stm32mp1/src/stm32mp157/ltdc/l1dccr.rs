///Register `L1DCCR` reader
pub type R = crate::R<L1DCCRrs>;
///Register `L1DCCR` writer
pub type W = crate::W<L1DCCRrs>;
///Field `DCBLUE` reader - DCBLUE
pub type DCBLUE_R = crate::FieldReader;
///Field `DCBLUE` writer - DCBLUE
pub type DCBLUE_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `DCGREEN` reader - DCGREEN
pub type DCGREEN_R = crate::FieldReader;
///Field `DCGREEN` writer - DCGREEN
pub type DCGREEN_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `DCRED` reader - DCRED
pub type DCRED_R = crate::FieldReader;
///Field `DCRED` writer - DCRED
pub type DCRED_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `DCALPHA` reader - DCALPHA
pub type DCALPHA_R = crate::FieldReader;
///Field `DCALPHA` writer - DCALPHA
pub type DCALPHA_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    ///Bits 0:7 - DCBLUE
    #[inline(always)]
    pub fn dcblue(&self) -> DCBLUE_R {
        DCBLUE_R::new((self.bits & 0xff) as u8)
    }
    ///Bits 8:15 - DCGREEN
    #[inline(always)]
    pub fn dcgreen(&self) -> DCGREEN_R {
        DCGREEN_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    ///Bits 16:23 - DCRED
    #[inline(always)]
    pub fn dcred(&self) -> DCRED_R {
        DCRED_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    ///Bits 24:31 - DCALPHA
    #[inline(always)]
    pub fn dcalpha(&self) -> DCALPHA_R {
        DCALPHA_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("L1DCCR")
            .field("dcblue", &self.dcblue())
            .field("dcgreen", &self.dcgreen())
            .field("dcred", &self.dcred())
            .field("dcalpha", &self.dcalpha())
            .finish()
    }
}
impl W {
    ///Bits 0:7 - DCBLUE
    #[inline(always)]
    pub fn dcblue(&mut self) -> DCBLUE_W<'_, L1DCCRrs> {
        DCBLUE_W::new(self, 0)
    }
    ///Bits 8:15 - DCGREEN
    #[inline(always)]
    pub fn dcgreen(&mut self) -> DCGREEN_W<'_, L1DCCRrs> {
        DCGREEN_W::new(self, 8)
    }
    ///Bits 16:23 - DCRED
    #[inline(always)]
    pub fn dcred(&mut self) -> DCRED_W<'_, L1DCCRrs> {
        DCRED_W::new(self, 16)
    }
    ///Bits 24:31 - DCALPHA
    #[inline(always)]
    pub fn dcalpha(&mut self) -> DCALPHA_W<'_, L1DCCRrs> {
        DCALPHA_W::new(self, 24)
    }
}
/**This register defines the default color of a layer in the format ARGB. The default color is used outside the defined layer window or when a layer is disabled. The reset value of 0x00000000 defines a transparent black color.

You can [`read`](crate::Reg::read) this register and get [`l1dccr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`l1dccr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#LTDC:L1DCCR)*/
pub struct L1DCCRrs;
impl crate::RegisterSpec for L1DCCRrs {
    type Ux = u32;
}
///`read()` method returns [`l1dccr::R`](R) reader structure
impl crate::Readable for L1DCCRrs {}
///`write(|w| ..)` method takes [`l1dccr::W`](W) writer structure
impl crate::Writable for L1DCCRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets L1DCCR to value 0
impl crate::Resettable for L1DCCRrs {}
