///Register `CR` reader
pub type R = crate::R<CRrs>;
///Register `CR` writer
pub type W = crate::W<CRrs>;
///Field `T` reader - T
pub type T_R = crate::FieldReader;
///Field `T` writer - T
pub type T_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
///Field `WDGA` reader - WDGA
pub type WDGA_R = crate::BitReader;
///Field `WDGA` writer - WDGA
pub type WDGA_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bits 0:6 - T
    #[inline(always)]
    pub fn t(&self) -> T_R {
        T_R::new((self.bits & 0x7f) as u8)
    }
    ///Bit 7 - WDGA
    #[inline(always)]
    pub fn wdga(&self) -> WDGA_R {
        WDGA_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CR")
            .field("t", &self.t())
            .field("wdga", &self.wdga())
            .finish()
    }
}
impl W {
    ///Bits 0:6 - T
    #[inline(always)]
    pub fn t(&mut self) -> T_W<'_, CRrs> {
        T_W::new(self, 0)
    }
    ///Bit 7 - WDGA
    #[inline(always)]
    pub fn wdga(&mut self) -> WDGA_W<'_, CRrs> {
        WDGA_W::new(self, 7)
    }
}
/**Control register

You can [`read`](crate::Reg::read) this register and get [`cr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#WWDG1:CR)*/
pub struct CRrs;
impl crate::RegisterSpec for CRrs {
    type Ux = u16;
}
///`read()` method returns [`cr::R`](R) reader structure
impl crate::Readable for CRrs {}
///`write(|w| ..)` method takes [`cr::W`](W) writer structure
impl crate::Writable for CRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CR to value 0x7f
impl crate::Resettable for CRrs {
    const RESET_VALUE: u16 = 0x7f;
}
