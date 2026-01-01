///Register `MACMDIOAR` reader
pub type R = crate::R<MACMDIOARrs>;
///Register `MACMDIOAR` writer
pub type W = crate::W<MACMDIOARrs>;
///Field `MB` reader - MII Busy
pub type MB_R = crate::BitReader;
///Field `MB` writer - MII Busy
pub type MB_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `C45E` reader - Clause 45 PHY Enable
pub type C45E_R = crate::BitReader;
///Field `C45E` writer - Clause 45 PHY Enable
pub type C45E_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `GOC` reader - MII Operation Command
pub type GOC_R = crate::FieldReader;
///Field `GOC` writer - MII Operation Command
pub type GOC_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `SKAP` reader - Skip Address Packet
pub type SKAP_R = crate::BitReader;
///Field `SKAP` writer - Skip Address Packet
pub type SKAP_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CR` reader - CSR Clock Range
pub type CR_R = crate::FieldReader;
///Field `CR` writer - CSR Clock Range
pub type CR_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `NTC` reader - Number of Training Clocks
pub type NTC_R = crate::FieldReader;
///Field `NTC` writer - Number of Training Clocks
pub type NTC_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `RDA` reader - Register/Device Address
pub type RDA_R = crate::FieldReader;
///Field `RDA` writer - Register/Device Address
pub type RDA_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
///Field `PA` reader - Physical Layer Address
pub type PA_R = crate::FieldReader;
///Field `PA` writer - Physical Layer Address
pub type PA_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
///Field `BTB` reader - Back to Back transactions
pub type BTB_R = crate::BitReader;
///Field `BTB` writer - Back to Back transactions
pub type BTB_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PSE` reader - Preamble Suppression Enable
pub type PSE_R = crate::BitReader;
///Field `PSE` writer - Preamble Suppression Enable
pub type PSE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - MII Busy
    #[inline(always)]
    pub fn mb(&self) -> MB_R {
        MB_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Clause 45 PHY Enable
    #[inline(always)]
    pub fn c45e(&self) -> C45E_R {
        C45E_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bits 2:3 - MII Operation Command
    #[inline(always)]
    pub fn goc(&self) -> GOC_R {
        GOC_R::new(((self.bits >> 2) & 3) as u8)
    }
    ///Bit 4 - Skip Address Packet
    #[inline(always)]
    pub fn skap(&self) -> SKAP_R {
        SKAP_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bits 8:11 - CSR Clock Range
    #[inline(always)]
    pub fn cr(&self) -> CR_R {
        CR_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    ///Bits 12:14 - Number of Training Clocks
    #[inline(always)]
    pub fn ntc(&self) -> NTC_R {
        NTC_R::new(((self.bits >> 12) & 7) as u8)
    }
    ///Bits 16:20 - Register/Device Address
    #[inline(always)]
    pub fn rda(&self) -> RDA_R {
        RDA_R::new(((self.bits >> 16) & 0x1f) as u8)
    }
    ///Bits 21:25 - Physical Layer Address
    #[inline(always)]
    pub fn pa(&self) -> PA_R {
        PA_R::new(((self.bits >> 21) & 0x1f) as u8)
    }
    ///Bit 26 - Back to Back transactions
    #[inline(always)]
    pub fn btb(&self) -> BTB_R {
        BTB_R::new(((self.bits >> 26) & 1) != 0)
    }
    ///Bit 27 - Preamble Suppression Enable
    #[inline(always)]
    pub fn pse(&self) -> PSE_R {
        PSE_R::new(((self.bits >> 27) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MACMDIOAR")
            .field("mb", &self.mb())
            .field("c45e", &self.c45e())
            .field("goc", &self.goc())
            .field("skap", &self.skap())
            .field("cr", &self.cr())
            .field("ntc", &self.ntc())
            .field("rda", &self.rda())
            .field("pa", &self.pa())
            .field("btb", &self.btb())
            .field("pse", &self.pse())
            .finish()
    }
}
impl W {
    ///Bit 0 - MII Busy
    #[inline(always)]
    pub fn mb(&mut self) -> MB_W<'_, MACMDIOARrs> {
        MB_W::new(self, 0)
    }
    ///Bit 1 - Clause 45 PHY Enable
    #[inline(always)]
    pub fn c45e(&mut self) -> C45E_W<'_, MACMDIOARrs> {
        C45E_W::new(self, 1)
    }
    ///Bits 2:3 - MII Operation Command
    #[inline(always)]
    pub fn goc(&mut self) -> GOC_W<'_, MACMDIOARrs> {
        GOC_W::new(self, 2)
    }
    ///Bit 4 - Skip Address Packet
    #[inline(always)]
    pub fn skap(&mut self) -> SKAP_W<'_, MACMDIOARrs> {
        SKAP_W::new(self, 4)
    }
    ///Bits 8:11 - CSR Clock Range
    #[inline(always)]
    pub fn cr(&mut self) -> CR_W<'_, MACMDIOARrs> {
        CR_W::new(self, 8)
    }
    ///Bits 12:14 - Number of Training Clocks
    #[inline(always)]
    pub fn ntc(&mut self) -> NTC_W<'_, MACMDIOARrs> {
        NTC_W::new(self, 12)
    }
    ///Bits 16:20 - Register/Device Address
    #[inline(always)]
    pub fn rda(&mut self) -> RDA_W<'_, MACMDIOARrs> {
        RDA_W::new(self, 16)
    }
    ///Bits 21:25 - Physical Layer Address
    #[inline(always)]
    pub fn pa(&mut self) -> PA_W<'_, MACMDIOARrs> {
        PA_W::new(self, 21)
    }
    ///Bit 26 - Back to Back transactions
    #[inline(always)]
    pub fn btb(&mut self) -> BTB_W<'_, MACMDIOARrs> {
        BTB_W::new(self, 26)
    }
    ///Bit 27 - Preamble Suppression Enable
    #[inline(always)]
    pub fn pse(&mut self) -> PSE_W<'_, MACMDIOARrs> {
        PSE_W::new(self, 27)
    }
}
/**MDIO address register

You can [`read`](crate::Reg::read) this register and get [`macmdioar::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`macmdioar::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H733.html#Ethernet_MAC:MACMDIOAR)*/
pub struct MACMDIOARrs;
impl crate::RegisterSpec for MACMDIOARrs {
    type Ux = u32;
}
///`read()` method returns [`macmdioar::R`](R) reader structure
impl crate::Readable for MACMDIOARrs {}
///`write(|w| ..)` method takes [`macmdioar::W`](W) writer structure
impl crate::Writable for MACMDIOARrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets MACMDIOAR to value 0
impl crate::Resettable for MACMDIOARrs {}
