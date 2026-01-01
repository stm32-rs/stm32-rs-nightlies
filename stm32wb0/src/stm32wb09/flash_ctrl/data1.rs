///Register `DATA1` reader
pub type R = crate::R<DATA1rs>;
///Register `DATA1` writer
pub type W = crate::W<DATA1rs>;
///Field `DATA1` reader - Value to be used as DATA for any COMMAND of type WRITE
pub type DATA1_R = crate::FieldReader<u32>;
///Field `DATA1` writer - Value to be used as DATA for any COMMAND of type WRITE
pub type DATA1_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - Value to be used as DATA for any COMMAND of type WRITE
    #[inline(always)]
    pub fn data1(&self) -> DATA1_R {
        DATA1_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DATA1")
            .field("data1", &self.data1())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - Value to be used as DATA for any COMMAND of type WRITE
    #[inline(always)]
    pub fn data1(&mut self) -> DATA1_W<'_, DATA1rs> {
        DATA1_W::new(self, 0)
    }
}
/**DATA1 register

You can [`read`](crate::Reg::read) this register and get [`data1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`data1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB09.html#FLASH_CTRL:DATA1)*/
pub struct DATA1rs;
impl crate::RegisterSpec for DATA1rs {
    type Ux = u32;
}
///`read()` method returns [`data1::R`](R) reader structure
impl crate::Readable for DATA1rs {}
///`write(|w| ..)` method takes [`data1::W`](W) writer structure
impl crate::Writable for DATA1rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets DATA1 to value 0xffff_ffff
impl crate::Resettable for DATA1rs {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
