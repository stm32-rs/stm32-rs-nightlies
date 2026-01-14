///Register `MACSSIR` reader
pub type R = crate::R<MACSSIRrs>;
///Register `MACSSIR` writer
pub type W = crate::W<MACSSIRrs>;
///Field `SNSINC` reader - SNSINC
pub type SNSINC_R = crate::FieldReader;
///Field `SNSINC` writer - SNSINC
pub type SNSINC_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `SSINC` reader - SSINC
pub type SSINC_R = crate::FieldReader;
///Field `SSINC` writer - SSINC
pub type SSINC_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    ///Bits 8:15 - SNSINC
    #[inline(always)]
    pub fn snsinc(&self) -> SNSINC_R {
        SNSINC_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    ///Bits 16:23 - SSINC
    #[inline(always)]
    pub fn ssinc(&self) -> SSINC_R {
        SSINC_R::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MACSSIR")
            .field("snsinc", &self.snsinc())
            .field("ssinc", &self.ssinc())
            .finish()
    }
}
impl W {
    ///Bits 8:15 - SNSINC
    #[inline(always)]
    pub fn snsinc(&mut self) -> SNSINC_W<'_, MACSSIRrs> {
        SNSINC_W::new(self, 8)
    }
    ///Bits 16:23 - SSINC
    #[inline(always)]
    pub fn ssinc(&mut self) -> SSINC_W<'_, MACSSIRrs> {
        SSINC_W::new(self, 16)
    }
}
/**The Sub-second Increment register is present only when the IEEE 1588 timestamp feature is selected without an external timestamp input. In Coarse Update mode \[Bit 1 in ETH_MACTSCR register, the value in this register is added to the system time every clock cycle of HCLK. In Fine Update mode, the value in this register is added to the system time whenever the Accumulator gets an overflow.

You can [`read`](crate::Reg::read) this register and get [`macssir::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`macssir::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#ETH_MAC_MMC:MACSSIR)*/
pub struct MACSSIRrs;
impl crate::RegisterSpec for MACSSIRrs {
    type Ux = u32;
}
///`read()` method returns [`macssir::R`](R) reader structure
impl crate::Readable for MACSSIRrs {}
///`write(|w| ..)` method takes [`macssir::W`](W) writer structure
impl crate::Writable for MACSSIRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets MACSSIR to value 0
impl crate::Resettable for MACSSIRrs {}
