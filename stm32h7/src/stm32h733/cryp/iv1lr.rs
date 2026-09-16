///Register `IV1LR` reader
pub type R = crate::R<IV1LRrs>;
///Register `IV1LR` writer
pub type W = crate::W<IV1LRrs>;
///Field `IV` reader - IV95
pub type IV_R = crate::FieldReader<u32>;
///Field `IV` writer - IV95
pub type IV_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - IV95
    #[inline(always)]
    pub fn iv(&self) -> IV_R {
        IV_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IV1LR").field("iv", &self.iv()).finish()
    }
}
impl W {
    ///Bits 0:31 - IV95
    #[inline(always)]
    pub fn iv(&mut self) -> IV_W<'_, IV1LRrs> {
        IV_W::new(self, 0)
    }
}
/**Initialization vector register 1L

You can [`read`](crate::Reg::read) this register and get [`iv1lr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`iv1lr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H733.html#CRYP:IV1LR)*/
pub struct IV1LRrs;
impl crate::RegisterSpec for IV1LRrs {
    type Ux = u32;
}
///`read()` method returns [`iv1lr::R`](R) reader structure
impl crate::Readable for IV1LRrs {}
///`write(|w| ..)` method takes [`iv1lr::W`](W) writer structure
impl crate::Writable for IV1LRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets IV1LR to value 0
impl crate::Resettable for IV1LRrs {}
