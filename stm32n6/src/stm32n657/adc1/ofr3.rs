///Register `OFR3` reader
pub type R = crate::R<OFR3rs>;
///Register `OFR3` writer
pub type W = crate::W<OFR3rs>;
///Field `OFFSET` reader - Data offset y for the channel programmed into OFFSETy_CH\[4:0\] bits
pub type OFFSET_R = crate::FieldReader<u32>;
///Field `OFFSET` writer - Data offset y for the channel programmed into OFFSETy_CH\[4:0\] bits
pub type OFFSET_W<'a, REG> = crate::FieldWriter<'a, REG, 22, u32>;
impl R {
    ///Bits 0:21 - Data offset y for the channel programmed into OFFSETy_CH\[4:0\] bits
    #[inline(always)]
    pub fn offset(&self) -> OFFSET_R {
        OFFSET_R::new(self.bits & 0x003f_ffff)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("OFR3")
            .field("offset", &self.offset())
            .finish()
    }
}
impl W {
    ///Bits 0:21 - Data offset y for the channel programmed into OFFSETy_CH\[4:0\] bits
    #[inline(always)]
    pub fn offset(&mut self) -> OFFSET_W<'_, OFR3rs> {
        OFFSET_W::new(self, 0)
    }
}
/**ADC offset 3 register

You can [`read`](crate::Reg::read) this register and get [`ofr3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ofr3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#ADC1:OFR3)*/
pub struct OFR3rs;
impl crate::RegisterSpec for OFR3rs {
    type Ux = u32;
}
///`read()` method returns [`ofr3::R`](R) reader structure
impl crate::Readable for OFR3rs {}
///`write(|w| ..)` method takes [`ofr3::W`](W) writer structure
impl crate::Writable for OFR3rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets OFR3 to value 0
impl crate::Resettable for OFR3rs {}
