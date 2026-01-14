///Register `FS_GOTGCTL` reader
pub type R = crate::R<FS_GOTGCTLrs>;
///Register `FS_GOTGCTL` writer
pub type W = crate::W<FS_GOTGCTLrs>;
///Field `SRQSCS` reader - Session request success
pub type SRQSCS_R = crate::BitReader;
///Field `SRQ` reader - Session request
pub type SRQ_R = crate::BitReader;
///Field `SRQ` writer - Session request
pub type SRQ_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `HNGSCS` reader - Host negotiation success
pub type HNGSCS_R = crate::BitReader;
///Field `HNPRQ` reader - HNP request
pub type HNPRQ_R = crate::BitReader;
///Field `HNPRQ` writer - HNP request
pub type HNPRQ_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `HSHNPEN` reader - Host set HNP enable
pub type HSHNPEN_R = crate::BitReader;
///Field `HSHNPEN` writer - Host set HNP enable
pub type HSHNPEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DHNPEN` reader - Device HNP enabled
pub type DHNPEN_R = crate::BitReader;
///Field `DHNPEN` writer - Device HNP enabled
pub type DHNPEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CIDSTS` reader - Connector ID status
pub type CIDSTS_R = crate::BitReader;
///Field `DBCT` reader - Long/short debounce time
pub type DBCT_R = crate::BitReader;
///Field `ASVLD` reader - A-session valid
pub type ASVLD_R = crate::BitReader;
///Field `BSVLD` reader - B-session valid
pub type BSVLD_R = crate::BitReader;
impl R {
    ///Bit 0 - Session request success
    #[inline(always)]
    pub fn srqscs(&self) -> SRQSCS_R {
        SRQSCS_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Session request
    #[inline(always)]
    pub fn srq(&self) -> SRQ_R {
        SRQ_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 8 - Host negotiation success
    #[inline(always)]
    pub fn hngscs(&self) -> HNGSCS_R {
        HNGSCS_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - HNP request
    #[inline(always)]
    pub fn hnprq(&self) -> HNPRQ_R {
        HNPRQ_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - Host set HNP enable
    #[inline(always)]
    pub fn hshnpen(&self) -> HSHNPEN_R {
        HSHNPEN_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - Device HNP enabled
    #[inline(always)]
    pub fn dhnpen(&self) -> DHNPEN_R {
        DHNPEN_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 16 - Connector ID status
    #[inline(always)]
    pub fn cidsts(&self) -> CIDSTS_R {
        CIDSTS_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - Long/short debounce time
    #[inline(always)]
    pub fn dbct(&self) -> DBCT_R {
        DBCT_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - A-session valid
    #[inline(always)]
    pub fn asvld(&self) -> ASVLD_R {
        ASVLD_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 19 - B-session valid
    #[inline(always)]
    pub fn bsvld(&self) -> BSVLD_R {
        BSVLD_R::new(((self.bits >> 19) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FS_GOTGCTL")
            .field("srqscs", &self.srqscs())
            .field("srq", &self.srq())
            .field("hngscs", &self.hngscs())
            .field("hnprq", &self.hnprq())
            .field("hshnpen", &self.hshnpen())
            .field("dhnpen", &self.dhnpen())
            .field("cidsts", &self.cidsts())
            .field("dbct", &self.dbct())
            .field("asvld", &self.asvld())
            .field("bsvld", &self.bsvld())
            .finish()
    }
}
impl W {
    ///Bit 1 - Session request
    #[inline(always)]
    pub fn srq(&mut self) -> SRQ_W<'_, FS_GOTGCTLrs> {
        SRQ_W::new(self, 1)
    }
    ///Bit 9 - HNP request
    #[inline(always)]
    pub fn hnprq(&mut self) -> HNPRQ_W<'_, FS_GOTGCTLrs> {
        HNPRQ_W::new(self, 9)
    }
    ///Bit 10 - Host set HNP enable
    #[inline(always)]
    pub fn hshnpen(&mut self) -> HSHNPEN_W<'_, FS_GOTGCTLrs> {
        HSHNPEN_W::new(self, 10)
    }
    ///Bit 11 - Device HNP enabled
    #[inline(always)]
    pub fn dhnpen(&mut self) -> DHNPEN_W<'_, FS_GOTGCTLrs> {
        DHNPEN_W::new(self, 11)
    }
}
/**OTG_FS control and status register (OTG_FS_GOTGCTL)

You can [`read`](crate::Reg::read) this register and get [`fs_gotgctl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fs_gotgctl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F417.html#OTG_FS_GLOBAL:FS_GOTGCTL)*/
pub struct FS_GOTGCTLrs;
impl crate::RegisterSpec for FS_GOTGCTLrs {
    type Ux = u32;
}
///`read()` method returns [`fs_gotgctl::R`](R) reader structure
impl crate::Readable for FS_GOTGCTLrs {}
///`write(|w| ..)` method takes [`fs_gotgctl::W`](W) writer structure
impl crate::Writable for FS_GOTGCTLrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets FS_GOTGCTL to value 0x0800
impl crate::Resettable for FS_GOTGCTLrs {
    const RESET_VALUE: u32 = 0x0800;
}
