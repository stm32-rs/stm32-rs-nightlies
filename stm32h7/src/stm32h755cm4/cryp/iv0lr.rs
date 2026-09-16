///Register `IV0LR` reader
pub type R = crate::R<IV0LRrs>;
///Register `IV0LR` writer
pub type W = crate::W<IV0LRrs>;
///Field `IV` reader - IV31
pub type IV_R = crate::FieldReader<u32>;
///Field `IV` writer - IV31
pub type IV_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - IV31
    #[inline(always)]
    pub fn iv(&self) -> IV_R {
        IV_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IV0LR").field("iv", &self.iv()).finish()
    }
}
impl W {
    ///Bits 0:31 - IV31
    #[inline(always)]
    pub fn iv(&mut self) -> IV_W<'_, IV0LRrs> {
        IV_W::new(self, 0)
    }
}
/**initialization vector registers

You can [`read`](crate::Reg::read) this register and get [`iv0lr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`iv0lr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H755_CM4.html#CRYP:IV0LR)*/
pub struct IV0LRrs;
impl crate::RegisterSpec for IV0LRrs {
    type Ux = u32;
}
///`read()` method returns [`iv0lr::R`](R) reader structure
impl crate::Readable for IV0LRrs {}
///`write(|w| ..)` method takes [`iv0lr::W`](W) writer structure
impl crate::Writable for IV0LRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets IV0LR to value 0
impl crate::Resettable for IV0LRrs {}
