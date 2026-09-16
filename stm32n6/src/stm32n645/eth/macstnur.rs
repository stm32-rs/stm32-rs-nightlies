///Register `MACSTNUR` reader
pub type R = crate::R<MACSTNURrs>;
///Register `MACSTNUR` writer
pub type W = crate::W<MACSTNURrs>;
///Field `TSSS` reader - Timestamp subseconds
pub type TSSS_R = crate::FieldReader<u32>;
///Field `TSSS` writer - Timestamp subseconds
pub type TSSS_W<'a, REG> = crate::FieldWriter<'a, REG, 31, u32>;
///Field `ADDSUB` reader - Add or Subtract Time
pub type ADDSUB_R = crate::BitReader;
///Field `ADDSUB` writer - Add or Subtract Time
pub type ADDSUB_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bits 0:30 - Timestamp subseconds
    #[inline(always)]
    pub fn tsss(&self) -> TSSS_R {
        TSSS_R::new(self.bits & 0x7fff_ffff)
    }
    ///Bit 31 - Add or Subtract Time
    #[inline(always)]
    pub fn addsub(&self) -> ADDSUB_R {
        ADDSUB_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MACSTNUR")
            .field("tsss", &self.tsss())
            .field("addsub", &self.addsub())
            .finish()
    }
}
impl W {
    ///Bits 0:30 - Timestamp subseconds
    #[inline(always)]
    pub fn tsss(&mut self) -> TSSS_W<'_, MACSTNURrs> {
        TSSS_W::new(self, 0)
    }
    ///Bit 31 - Add or Subtract Time
    #[inline(always)]
    pub fn addsub(&mut self) -> ADDSUB_W<'_, MACSTNURrs> {
        ADDSUB_W::new(self, 31)
    }
}
/**System time nanoseconds update register

You can [`read`](crate::Reg::read) this register and get [`macstnur::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`macstnur::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#ETH:MACSTNUR)*/
pub struct MACSTNURrs;
impl crate::RegisterSpec for MACSTNURrs {
    type Ux = u32;
}
///`read()` method returns [`macstnur::R`](R) reader structure
impl crate::Readable for MACSTNURrs {}
///`write(|w| ..)` method takes [`macstnur::W`](W) writer structure
impl crate::Writable for MACSTNURrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets MACSTNUR to value 0
impl crate::Resettable for MACSTNURrs {}
