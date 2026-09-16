///Register `DLY4CR` reader
pub type R = crate::R<DLY4CRrs>;
///Register `DLY4CR` writer
pub type W = crate::W<DLY4CRrs>;
///Field `SKPDLY` reader - Delay to apply to a bitstream
pub type SKPDLY_R = crate::FieldReader;
///Field `SKPDLY` writer - Delay to apply to a bitstream
pub type SKPDLY_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
///Field `SKPBF` reader - Skip busy flag
pub type SKPBF_R = crate::BitReader;
impl R {
    ///Bits 0:6 - Delay to apply to a bitstream
    #[inline(always)]
    pub fn skpdly(&self) -> SKPDLY_R {
        SKPDLY_R::new((self.bits & 0x7f) as u8)
    }
    ///Bit 31 - Skip busy flag
    #[inline(always)]
    pub fn skpbf(&self) -> SKPBF_R {
        SKPBF_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DLY4CR")
            .field("skpdly", &self.skpdly())
            .field("skpbf", &self.skpbf())
            .finish()
    }
}
impl W {
    ///Bits 0:6 - Delay to apply to a bitstream
    #[inline(always)]
    pub fn skpdly(&mut self) -> SKPDLY_W<'_, DLY4CRrs> {
        SKPDLY_W::new(self, 0)
    }
}
/**MDF delay control register 4

You can [`read`](crate::Reg::read) this register and get [`dly4cr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dly4cr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#MDF1:DLY4CR)*/
pub struct DLY4CRrs;
impl crate::RegisterSpec for DLY4CRrs {
    type Ux = u32;
}
///`read()` method returns [`dly4cr::R`](R) reader structure
impl crate::Readable for DLY4CRrs {}
///`write(|w| ..)` method takes [`dly4cr::W`](W) writer structure
impl crate::Writable for DLY4CRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets DLY4CR to value 0
impl crate::Resettable for DLY4CRrs {}
