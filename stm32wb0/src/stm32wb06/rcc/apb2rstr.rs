///Register `APB2RSTR` reader
pub type R = crate::R<APB2RSTRrs>;
///Register `APB2RSTR` writer
pub type W = crate::W<APB2RSTRrs>;
///Field `MRBLERST` reader - MR_BLE (Bluetooth radio) reset.
pub type MRBLERST_R = crate::BitReader;
///Field `MRBLERST` writer - MR_BLE (Bluetooth radio) reset.
pub type MRBLERST_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - MR_BLE (Bluetooth radio) reset.
    #[inline(always)]
    pub fn mrblerst(&self) -> MRBLERST_R {
        MRBLERST_R::new((self.bits & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("APB2RSTR")
            .field("mrblerst", &self.mrblerst())
            .finish()
    }
}
impl W {
    ///Bit 0 - MR_BLE (Bluetooth radio) reset.
    #[inline(always)]
    pub fn mrblerst(&mut self) -> MRBLERST_W<'_, APB2RSTRrs> {
        MRBLERST_W::new(self, 0)
    }
}
/**APB2RSTR register

You can [`read`](crate::Reg::read) this register and get [`apb2rstr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb2rstr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB06.html#RCC:APB2RSTR)*/
pub struct APB2RSTRrs;
impl crate::RegisterSpec for APB2RSTRrs {
    type Ux = u32;
}
///`read()` method returns [`apb2rstr::R`](R) reader structure
impl crate::Readable for APB2RSTRrs {}
///`write(|w| ..)` method takes [`apb2rstr::W`](W) writer structure
impl crate::Writable for APB2RSTRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets APB2RSTR to value 0
impl crate::Resettable for APB2RSTRrs {}
