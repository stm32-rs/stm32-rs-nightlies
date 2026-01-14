///Register `DR16` reader
pub type R = crate::R<DR16rs>;
///Register `DR16` writer
pub type W = crate::W<DR16rs>;
///Field `DATA` reader - Data
pub type DATA_R = crate::FieldReader<u16>;
///Field `DATA` writer - Data
pub type DATA_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16, crate::Safe>;
impl R {
    ///Bits 0:15 - Data
    #[inline(always)]
    pub fn data(&self) -> DATA_R {
        DATA_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DR16").field("data", &self.data()).finish()
    }
}
impl W {
    ///Bits 0:15 - Data
    #[inline(always)]
    pub fn data(&mut self) -> DATA_W<'_, DR16rs> {
        DATA_W::new(self, 0)
    }
}
/**Data register: half word (16 bit) access

You can [`read`](crate::Reg::read) this register and get [`dr16::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dr16::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F769.html#QUADSPI:DR16)*/
pub struct DR16rs;
impl crate::RegisterSpec for DR16rs {
    type Ux = u16;
}
///`read()` method returns [`dr16::R`](R) reader structure
impl crate::Readable for DR16rs {}
///`write(|w| ..)` method takes [`dr16::W`](W) writer structure
impl crate::Writable for DR16rs {
    type Safety = crate::Safe;
}
///`reset()` method sets DR16 to value 0
impl crate::Resettable for DR16rs {}
