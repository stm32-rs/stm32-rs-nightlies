///Register `CTR` reader
pub type R = crate::R<CTRrs>;
///Register `CTR` writer
pub type W = crate::W<CTRrs>;
///Field `EN` reader - Cache enable
pub type EN_R = crate::BitReader;
///Field `EN` writer - Cache enable
pub type EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PCACHEADDR` reader - Cacheable page index
pub type PCACHEADDR_R = crate::FieldReader<u16>;
///Field `PCACHEADDR` writer - Cacheable page index
pub type PCACHEADDR_W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    ///Bit 0 - Cache enable
    #[inline(always)]
    pub fn en(&self) -> EN_R {
        EN_R::new((self.bits & 1) != 0)
    }
    ///Bits 8:19 - Cacheable page index
    #[inline(always)]
    pub fn pcacheaddr(&self) -> PCACHEADDR_R {
        PCACHEADDR_R::new(((self.bits >> 8) & 0x0fff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CTR")
            .field("en", &self.en())
            .field("pcacheaddr", &self.pcacheaddr())
            .finish()
    }
}
impl W {
    ///Bit 0 - Cache enable
    #[inline(always)]
    pub fn en(&mut self) -> EN_W<'_, CTRrs> {
        EN_W::new(self, 0)
    }
    ///Bits 8:19 - Cacheable page index
    #[inline(always)]
    pub fn pcacheaddr(&mut self) -> PCACHEADDR_W<'_, CTRrs> {
        PCACHEADDR_W::new(self, 8)
    }
}
/**control register

You can [`read`](crate::Reg::read) this register and get [`ctr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H757_CM4.html#ART:CTR)*/
pub struct CTRrs;
impl crate::RegisterSpec for CTRrs {
    type Ux = u32;
}
///`read()` method returns [`ctr::R`](R) reader structure
impl crate::Readable for CTRrs {}
///`write(|w| ..)` method takes [`ctr::W`](W) writer structure
impl crate::Writable for CTRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CTR to value 0x04
impl crate::Resettable for CTRrs {
    const RESET_VALUE: u32 = 0x04;
}
