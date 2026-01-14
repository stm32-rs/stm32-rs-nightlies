///Register `FLTR` reader
pub type R = crate::R<FLTRrs>;
///Register `FLTR` writer
pub type W = crate::W<FLTRrs>;
///Field `DNF` reader - Digital noise filter
pub type DNF_R = crate::FieldReader;
///Field `DNF` writer - Digital noise filter
pub type DNF_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `ANOFF` reader - Analog noise filter OFF
pub type ANOFF_R = crate::BitReader;
///Field `ANOFF` writer - Analog noise filter OFF
pub type ANOFF_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bits 0:3 - Digital noise filter
    #[inline(always)]
    pub fn dnf(&self) -> DNF_R {
        DNF_R::new((self.bits & 0x0f) as u8)
    }
    ///Bit 4 - Analog noise filter OFF
    #[inline(always)]
    pub fn anoff(&self) -> ANOFF_R {
        ANOFF_R::new(((self.bits >> 4) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FLTR")
            .field("dnf", &self.dnf())
            .field("anoff", &self.anoff())
            .finish()
    }
}
impl W {
    ///Bits 0:3 - Digital noise filter
    #[inline(always)]
    pub fn dnf(&mut self) -> DNF_W<'_, FLTRrs> {
        DNF_W::new(self, 0)
    }
    ///Bit 4 - Analog noise filter OFF
    #[inline(always)]
    pub fn anoff(&mut self) -> ANOFF_W<'_, FLTRrs> {
        ANOFF_W::new(self, 4)
    }
}
/**I2C FLTR register

You can [`read`](crate::Reg::read) this register and get [`fltr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fltr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F439.html#I2C3:FLTR)*/
pub struct FLTRrs;
impl crate::RegisterSpec for FLTRrs {
    type Ux = u32;
}
///`read()` method returns [`fltr::R`](R) reader structure
impl crate::Readable for FLTRrs {}
///`write(|w| ..)` method takes [`fltr::W`](W) writer structure
impl crate::Writable for FLTRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets FLTR to value 0
impl crate::Resettable for FLTRrs {}
