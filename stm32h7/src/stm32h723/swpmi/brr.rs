///Register `BRR` reader
pub type R = crate::R<BRRrs>;
///Register `BRR` writer
pub type W = crate::W<BRRrs>;
///Field `BR` reader - Bitrate prescaler
pub type BR_R = crate::FieldReader;
///Field `BR` writer - Bitrate prescaler
pub type BR_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    ///Bits 0:7 - Bitrate prescaler
    #[inline(always)]
    pub fn br(&self) -> BR_R {
        BR_R::new((self.bits & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BRR").field("br", &self.br()).finish()
    }
}
impl W {
    ///Bits 0:7 - Bitrate prescaler
    #[inline(always)]
    pub fn br(&mut self) -> BR_W<'_, BRRrs> {
        BR_W::new(self, 0)
    }
}
/**SWPMI Bitrate register

You can [`read`](crate::Reg::read) this register and get [`brr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`brr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H723.html#SWPMI:BRR)*/
pub struct BRRrs;
impl crate::RegisterSpec for BRRrs {
    type Ux = u32;
}
///`read()` method returns [`brr::R`](R) reader structure
impl crate::Readable for BRRrs {}
///`write(|w| ..)` method takes [`brr::W`](W) writer structure
impl crate::Writable for BRRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets BRR to value 0x01
impl crate::Resettable for BRRrs {
    const RESET_VALUE: u32 = 0x01;
}
