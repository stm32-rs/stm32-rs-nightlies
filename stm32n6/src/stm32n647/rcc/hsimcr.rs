///Register `HSIMCR` reader
pub type R = crate::R<HSIMCRrs>;
///Register `HSIMCR` writer
pub type W = crate::W<HSIMCRrs>;
///Field `HSIREF` reader - HSI clock cycle counter reference value.
pub type HSIREF_R = crate::FieldReader<u16>;
///Field `HSIREF` writer - HSI clock cycle counter reference value.
pub type HSIREF_W<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
///Field `HSIDEV` reader - HSI clock count deviation value
pub type HSIDEV_R = crate::FieldReader;
///Field `HSIDEV` writer - HSI clock count deviation value
pub type HSIDEV_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
///Field `HSIMONEN` reader - HSI clock period monitor enable
pub type HSIMONEN_R = crate::BitReader;
///Field `HSIMONEN` writer - HSI clock period monitor enable
pub type HSIMONEN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bits 0:10 - HSI clock cycle counter reference value.
    #[inline(always)]
    pub fn hsiref(&self) -> HSIREF_R {
        HSIREF_R::new((self.bits & 0x07ff) as u16)
    }
    ///Bits 16:21 - HSI clock count deviation value
    #[inline(always)]
    pub fn hsidev(&self) -> HSIDEV_R {
        HSIDEV_R::new(((self.bits >> 16) & 0x3f) as u8)
    }
    ///Bit 31 - HSI clock period monitor enable
    #[inline(always)]
    pub fn hsimonen(&self) -> HSIMONEN_R {
        HSIMONEN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HSIMCR")
            .field("hsiref", &self.hsiref())
            .field("hsidev", &self.hsidev())
            .field("hsimonen", &self.hsimonen())
            .finish()
    }
}
impl W {
    ///Bits 0:10 - HSI clock cycle counter reference value.
    #[inline(always)]
    pub fn hsiref(&mut self) -> HSIREF_W<'_, HSIMCRrs> {
        HSIREF_W::new(self, 0)
    }
    ///Bits 16:21 - HSI clock count deviation value
    #[inline(always)]
    pub fn hsidev(&mut self) -> HSIDEV_W<'_, HSIMCRrs> {
        HSIDEV_W::new(self, 16)
    }
    ///Bit 31 - HSI clock period monitor enable
    #[inline(always)]
    pub fn hsimonen(&mut self) -> HSIMONEN_W<'_, HSIMCRrs> {
        HSIMONEN_W::new(self, 31)
    }
}
/**RCC HSI monitor control register

You can [`read`](crate::Reg::read) this register and get [`hsimcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hsimcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#RCC:HSIMCR)*/
pub struct HSIMCRrs;
impl crate::RegisterSpec for HSIMCRrs {
    type Ux = u32;
}
///`read()` method returns [`hsimcr::R`](R) reader structure
impl crate::Readable for HSIMCRrs {}
///`write(|w| ..)` method takes [`hsimcr::W`](W) writer structure
impl crate::Writable for HSIMCRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets HSIMCR to value 0x001f_07a1
impl crate::Resettable for HSIMCRrs {
    const RESET_VALUE: u32 = 0x001f_07a1;
}
