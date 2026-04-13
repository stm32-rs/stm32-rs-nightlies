///Register `DATA2` reader
pub type R = crate::R<DATA2rs>;
///Register `DATA2` writer
pub type W = crate::W<DATA2rs>;
///Field `DATA2` reader - Value to be used as DATA for any COMMAND of type WRITE
pub type DATA2_R = crate::FieldReader<u32>;
///Field `DATA2` writer - Value to be used as DATA for any COMMAND of type WRITE
pub type DATA2_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - Value to be used as DATA for any COMMAND of type WRITE
    #[inline(always)]
    pub fn data2(&self) -> DATA2_R {
        DATA2_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DATA2")
            .field("data2", &self.data2())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - Value to be used as DATA for any COMMAND of type WRITE
    #[inline(always)]
    pub fn data2(&mut self) -> DATA2_W<'_, DATA2rs> {
        DATA2_W::new(self, 0)
    }
}
/**DATA2 register

You can [`read`](crate::Reg::read) this register and get [`data2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`data2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB06.html#FLASH_CTRL:DATA2)*/
pub struct DATA2rs;
impl crate::RegisterSpec for DATA2rs {
    type Ux = u32;
}
///`read()` method returns [`data2::R`](R) reader structure
impl crate::Readable for DATA2rs {}
///`write(|w| ..)` method takes [`data2::W`](W) writer structure
impl crate::Writable for DATA2rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets DATA2 to value 0xffff_ffff
impl crate::Resettable for DATA2rs {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
