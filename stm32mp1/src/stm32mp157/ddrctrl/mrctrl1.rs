///Register `MRCTRL1` reader
pub type R = crate::R<MRCTRL1rs>;
///Register `MRCTRL1` writer
pub type W = crate::W<MRCTRL1rs>;
///Field `MR_DATA` reader - MR_DATA
pub type MR_DATA_R = crate::FieldReader<u16>;
///Field `MR_DATA` writer - MR_DATA
pub type MR_DATA_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    ///Bits 0:15 - MR_DATA
    #[inline(always)]
    pub fn mr_data(&self) -> MR_DATA_R {
        MR_DATA_R::new((self.bits & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MRCTRL1")
            .field("mr_data", &self.mr_data())
            .finish()
    }
}
impl W {
    ///Bits 0:15 - MR_DATA
    #[inline(always)]
    pub fn mr_data(&mut self) -> MR_DATA_W<'_, MRCTRL1rs> {
        MR_DATA_W::new(self, 0)
    }
}
/**DDRCTRL mode register read/write control register 1

You can [`read`](crate::Reg::read) this register and get [`mrctrl1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mrctrl1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#DDRCTRL:MRCTRL1)*/
pub struct MRCTRL1rs;
impl crate::RegisterSpec for MRCTRL1rs {
    type Ux = u32;
}
///`read()` method returns [`mrctrl1::R`](R) reader structure
impl crate::Readable for MRCTRL1rs {}
///`write(|w| ..)` method takes [`mrctrl1::W`](W) writer structure
impl crate::Writable for MRCTRL1rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets MRCTRL1 to value 0
impl crate::Resettable for MRCTRL1rs {}
