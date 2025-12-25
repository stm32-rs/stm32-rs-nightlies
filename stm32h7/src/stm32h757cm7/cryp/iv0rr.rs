///Register `IV0RR` reader
pub type R = crate::R<IV0RRrs>;
///Register `IV0RR` writer
pub type W = crate::W<IV0RRrs>;
///Field `IV` reader - IV63
pub type IV_R = crate::FieldReader<u32>;
///Field `IV` writer - IV63
pub type IV_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - IV63
    #[inline(always)]
    pub fn iv(&self) -> IV_R {
        IV_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IV0RR").field("iv", &self.iv()).finish()
    }
}
impl W {
    ///Bits 0:31 - IV63
    #[inline(always)]
    pub fn iv(&mut self) -> IV_W<'_, IV0RRrs> {
        IV_W::new(self, 0)
    }
}
/**initialization vector registers

You can [`read`](crate::Reg::read) this register and get [`iv0rr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`iv0rr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H757_CM7.html#CRYP:IV0RR)*/
pub struct IV0RRrs;
impl crate::RegisterSpec for IV0RRrs {
    type Ux = u32;
}
///`read()` method returns [`iv0rr::R`](R) reader structure
impl crate::Readable for IV0RRrs {}
///`write(|w| ..)` method takes [`iv0rr::W`](W) writer structure
impl crate::Writable for IV0RRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets IV0RR to value 0
impl crate::Resettable for IV0RRrs {}
