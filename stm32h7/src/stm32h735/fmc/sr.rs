#[doc = "Register `SR` reader"]
pub type R = crate::R<SRrs>;
#[doc = "Register `SR` writer"]
pub type W = crate::W<SRrs>;
#[doc = "Field `IRS` reader - Interrupt rising edge status The flag is set by hardware and reset by software. Note: If this bit is written by software to 1 it will be set."]
pub type IRS_R = crate::BitReader;
#[doc = "Field `IRS` writer - Interrupt rising edge status The flag is set by hardware and reset by software. Note: If this bit is written by software to 1 it will be set."]
pub type IRS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ILS` reader - Interrupt high-level status The flag is set by hardware and reset by software."]
pub type ILS_R = crate::BitReader;
#[doc = "Field `ILS` writer - Interrupt high-level status The flag is set by hardware and reset by software."]
pub type ILS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IFS` reader - Interrupt falling edge status The flag is set by hardware and reset by software. Note: If this bit is written by software to 1 it will be set."]
pub type IFS_R = crate::BitReader;
#[doc = "Field `IFS` writer - Interrupt falling edge status The flag is set by hardware and reset by software. Note: If this bit is written by software to 1 it will be set."]
pub type IFS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IREN` reader - Interrupt rising edge detection enable bit"]
pub type IREN_R = crate::BitReader;
#[doc = "Field `IREN` writer - Interrupt rising edge detection enable bit"]
pub type IREN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ILEN` reader - Interrupt high-level detection enable bit"]
pub type ILEN_R = crate::BitReader;
#[doc = "Field `ILEN` writer - Interrupt high-level detection enable bit"]
pub type ILEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IFEN` reader - Interrupt falling edge detection enable bit"]
pub type IFEN_R = crate::BitReader;
#[doc = "Field `IFEN` writer - Interrupt falling edge detection enable bit"]
pub type IFEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FEMPT` reader - FIFO empty. Read-only bit that provides the status of the FIFO"]
pub type FEMPT_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Interrupt rising edge status The flag is set by hardware and reset by software. Note: If this bit is written by software to 1 it will be set."]
    #[inline(always)]
    pub fn irs(&self) -> IRS_R {
        IRS_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Interrupt high-level status The flag is set by hardware and reset by software."]
    #[inline(always)]
    pub fn ils(&self) -> ILS_R {
        ILS_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Interrupt falling edge status The flag is set by hardware and reset by software. Note: If this bit is written by software to 1 it will be set."]
    #[inline(always)]
    pub fn ifs(&self) -> IFS_R {
        IFS_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Interrupt rising edge detection enable bit"]
    #[inline(always)]
    pub fn iren(&self) -> IREN_R {
        IREN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Interrupt high-level detection enable bit"]
    #[inline(always)]
    pub fn ilen(&self) -> ILEN_R {
        ILEN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Interrupt falling edge detection enable bit"]
    #[inline(always)]
    pub fn ifen(&self) -> IFEN_R {
        IFEN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - FIFO empty. Read-only bit that provides the status of the FIFO"]
    #[inline(always)]
    pub fn fempt(&self) -> FEMPT_R {
        FEMPT_R::new(((self.bits >> 6) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Interrupt rising edge status The flag is set by hardware and reset by software. Note: If this bit is written by software to 1 it will be set."]
    #[inline(always)]
    #[must_use]
    pub fn irs(&mut self) -> IRS_W<SRrs> {
        IRS_W::new(self, 0)
    }
    #[doc = "Bit 1 - Interrupt high-level status The flag is set by hardware and reset by software."]
    #[inline(always)]
    #[must_use]
    pub fn ils(&mut self) -> ILS_W<SRrs> {
        ILS_W::new(self, 1)
    }
    #[doc = "Bit 2 - Interrupt falling edge status The flag is set by hardware and reset by software. Note: If this bit is written by software to 1 it will be set."]
    #[inline(always)]
    #[must_use]
    pub fn ifs(&mut self) -> IFS_W<SRrs> {
        IFS_W::new(self, 2)
    }
    #[doc = "Bit 3 - Interrupt rising edge detection enable bit"]
    #[inline(always)]
    #[must_use]
    pub fn iren(&mut self) -> IREN_W<SRrs> {
        IREN_W::new(self, 3)
    }
    #[doc = "Bit 4 - Interrupt high-level detection enable bit"]
    #[inline(always)]
    #[must_use]
    pub fn ilen(&mut self) -> ILEN_W<SRrs> {
        ILEN_W::new(self, 4)
    }
    #[doc = "Bit 5 - Interrupt falling edge detection enable bit"]
    #[inline(always)]
    #[must_use]
    pub fn ifen(&mut self) -> IFEN_W<SRrs> {
        IFEN_W::new(self, 5)
    }
}
#[doc = "This register contains information about the FIFO status and interrupt. The FMC features a FIFO that is used when writing to memories to transfer up to 16 words of data.This is used to quickly write to the FIFO and free the AXI bus for transactions to peripherals other than the FMC, while the FMC is draining its FIFO into the memory. One of these register bits indicates the status of the FIFO, for ECC purposes.The ECC is calculated while the data are written to the memory. To read the correct ECC, the software must consequently wait until the FIFO is empty.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SRrs;
impl crate::RegisterSpec for SRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sr::R`](R) reader structure"]
impl crate::Readable for SRrs {}
#[doc = "`write(|w| ..)` method takes [`sr::W`](W) writer structure"]
impl crate::Writable for SRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SR to value 0x40"]
impl crate::Resettable for SRrs {
    const RESET_VALUE: u32 = 0x40;
}
