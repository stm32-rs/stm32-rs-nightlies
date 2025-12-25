///Register `TURCF` reader
pub type R = crate::R<TURCFrs>;
///Register `TURCF` writer
pub type W = crate::W<TURCFrs>;
///Field `NCL` reader - NCL
pub type NCL_R = crate::FieldReader<u16>;
///Field `NCL` writer - NCL
pub type NCL_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
///Field `DC` reader - DC
pub type DC_R = crate::FieldReader<u16>;
///Field `DC` writer - DC
pub type DC_W<'a, REG> = crate::FieldWriter<'a, REG, 14, u16>;
///Field `ELT` reader - ELT
pub type ELT_R = crate::BitReader;
///Field `ELT` writer - ELT
pub type ELT_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bits 0:15 - NCL
    #[inline(always)]
    pub fn ncl(&self) -> NCL_R {
        NCL_R::new((self.bits & 0xffff) as u16)
    }
    ///Bits 16:29 - DC
    #[inline(always)]
    pub fn dc(&self) -> DC_R {
        DC_R::new(((self.bits >> 16) & 0x3fff) as u16)
    }
    ///Bit 31 - ELT
    #[inline(always)]
    pub fn elt(&self) -> ELT_R {
        ELT_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TURCF")
            .field("ncl", &self.ncl())
            .field("dc", &self.dc())
            .field("elt", &self.elt())
            .finish()
    }
}
impl W {
    ///Bits 0:15 - NCL
    #[inline(always)]
    pub fn ncl(&mut self) -> NCL_W<'_, TURCFrs> {
        NCL_W::new(self, 0)
    }
    ///Bits 16:29 - DC
    #[inline(always)]
    pub fn dc(&mut self) -> DC_W<'_, TURCFrs> {
        DC_W::new(self, 16)
    }
    ///Bit 31 - ELT
    #[inline(always)]
    pub fn elt(&mut self) -> ELT_W<'_, TURCFrs> {
        ELT_W::new(self, 31)
    }
}
/**The length of the NTU is given by: NTU = CAN clock period x NC/DC. NC is an 18-bit value. Its high part, NCH\[17:16\] is hard wired to 0b01. Therefore the range of NC extends from 0x10000 to 0x1FFFF. The value configured by NCL is the initial value for FDCAN_TURNA.NAV\[15:0\]. DC is set to 0x1000 by hardware reset and it may not be written to 0x0000. Level 1: NC 4 * DC and NTU = CAN bit time Levels 0 and 2: NC 8 * DC The actual value of FDCAN_TUR may be changed by the clock drift compensation function of TTCAN level 0 and level 2 in order to adjust the node local view of the NTU to the time master view of the NTU. DC will not be changed by the automatic drift compensation, FDCAN_TURNA.NAV may be adjusted around NC in the range of the synchronization deviation limit given by FDCAN_TTOCF.LDSDL. NC and DC should be programmed to the largest suitable values in achieve the best computational accuracy for the drift compensation process.

You can [`read`](crate::Reg::read) this register and get [`turcf::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`turcf::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#FDCAN1:TURCF)*/
pub struct TURCFrs;
impl crate::RegisterSpec for TURCFrs {
    type Ux = u32;
}
///`read()` method returns [`turcf::R`](R) reader structure
impl crate::Readable for TURCFrs {}
///`write(|w| ..)` method takes [`turcf::W`](W) writer structure
impl crate::Writable for TURCFrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets TURCF to value 0
impl crate::Resettable for TURCFrs {}
