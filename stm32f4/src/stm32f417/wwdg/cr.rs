///Register `CR` reader
pub type R = crate::R<CRrs>;
///Register `CR` writer
pub type W = crate::W<CRrs>;
///Field `T` reader - 7-bit counter (MSB to LSB)
pub type T_R = crate::FieldReader;
///Field `T` writer - 7-bit counter (MSB to LSB)
pub type T_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
///Field `WDGA` reader - Activation bit
pub type WDGA_R = crate::BitReader;
///Field `WDGA` writer - Activation bit
pub type WDGA_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bits 0:6 - 7-bit counter (MSB to LSB)
    #[inline(always)]
    pub fn t(&self) -> T_R {
        T_R::new((self.bits & 0x7f) as u8)
    }
    ///Bit 7 - Activation bit
    #[inline(always)]
    pub fn wdga(&self) -> WDGA_R {
        WDGA_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CR")
            .field("wdga", &self.wdga())
            .field("t", &self.t())
            .finish()
    }
}
impl W {
    ///Bits 0:6 - 7-bit counter (MSB to LSB)
    #[inline(always)]
    pub fn t(&mut self) -> T_W<'_, CRrs> {
        T_W::new(self, 0)
    }
    ///Bit 7 - Activation bit
    #[inline(always)]
    pub fn wdga(&mut self) -> WDGA_W<'_, CRrs> {
        WDGA_W::new(self, 7)
    }
}
/**Control register

You can [`read`](crate::Reg::read) this register and get [`cr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F417.html#WWDG:CR)*/
pub struct CRrs;
impl crate::RegisterSpec for CRrs {
    type Ux = u32;
}
///`read()` method returns [`cr::R`](R) reader structure
impl crate::Readable for CRrs {}
///`write(|w| ..)` method takes [`cr::W`](W) writer structure
impl crate::Writable for CRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CR to value 0x7f
impl crate::Resettable for CRrs {
    const RESET_VALUE: u32 = 0x7f;
}
