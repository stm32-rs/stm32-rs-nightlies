///Register `RXGFC` reader
pub type R = crate::R<RXGFCrs>;
///Register `RXGFC` writer
pub type W = crate::W<RXGFCrs>;
///Field `RRFE` reader - Reject remote frames extended These are protected write (P) bits, which means that write access by the bits is possible only when the bit 1 \[CCE\] and bit 0 \[INIT\] of CCCR register are set to 1.
pub type RRFE_R = crate::BitReader;
///Field `RRFE` writer - Reject remote frames extended These are protected write (P) bits, which means that write access by the bits is possible only when the bit 1 \[CCE\] and bit 0 \[INIT\] of CCCR register are set to 1.
pub type RRFE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RRFS` reader - Reject remote frames standard These are protected write (P) bits, which means that write access by the bits is possible only when the bit 1 \[CCE\] and bit 0 \[INIT\] of CCCR register are set to 1.
pub type RRFS_R = crate::BitReader;
///Field `RRFS` writer - Reject remote frames standard These are protected write (P) bits, which means that write access by the bits is possible only when the bit 1 \[CCE\] and bit 0 \[INIT\] of CCCR register are set to 1.
pub type RRFS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ANFE` reader - Accept non-matching frames extended Defines how received messages with 29-bit IDs that do not match any element of the filter list are treated. These are protected write (P) bits, which means that write access by the bits is possible only when the bit 1 \[CCE\] and bit 0 \[INIT\] of CCCR register are set to 1.
pub type ANFE_R = crate::FieldReader;
///Field `ANFE` writer - Accept non-matching frames extended Defines how received messages with 29-bit IDs that do not match any element of the filter list are treated. These are protected write (P) bits, which means that write access by the bits is possible only when the bit 1 \[CCE\] and bit 0 \[INIT\] of CCCR register are set to 1.
pub type ANFE_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `ANFS` reader - Accept Non-matching frames standard Defines how received messages with 11-bit IDs that do not match any element of the filter list are treated. These are protected write (P) bits, which means that write access by the bits is possible only when the bit 1 \[CCE\] and bit 0 \[INIT\] of CCCR register are set to 1.
pub type ANFS_R = crate::FieldReader;
///Field `ANFS` writer - Accept Non-matching frames standard Defines how received messages with 11-bit IDs that do not match any element of the filter list are treated. These are protected write (P) bits, which means that write access by the bits is possible only when the bit 1 \[CCE\] and bit 0 \[INIT\] of CCCR register are set to 1.
pub type ANFS_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `F1OM` reader - FIFO 1 operation mode (overwrite or blocking) This is a protected write (P) bits, which means that write access by the bits is possible only when the bit 1 \[CCE\] and bit 0 \[INIT\] of CCCR register are set to 1.
pub type F1OM_R = crate::BitReader;
///Field `F1OM` writer - FIFO 1 operation mode (overwrite or blocking) This is a protected write (P) bits, which means that write access by the bits is possible only when the bit 1 \[CCE\] and bit 0 \[INIT\] of CCCR register are set to 1.
pub type F1OM_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `F0OM` reader - FIFO 0 operation mode (overwrite or blocking) This is protected write (P) bits, which means that write access by the bits is possible only when the bit 1 \[CCE\] and bit 0 \[INIT\] of CCCR register are set to 1.
pub type F0OM_R = crate::BitReader;
///Field `F0OM` writer - FIFO 0 operation mode (overwrite or blocking) This is protected write (P) bits, which means that write access by the bits is possible only when the bit 1 \[CCE\] and bit 0 \[INIT\] of CCCR register are set to 1.
pub type F0OM_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `LSS` reader - List size standard 1 to 28: Number of standard message ID filter elements >28: Values greater than 28 are interpreted as 28. These are protected write (P) bits, which means that write access by the bits is possible only when the bit 1 \[CCE\] and bit 0 \[INIT\] of CCCR register are set to 1.
pub type LSS_R = crate::FieldReader;
///Field `LSS` writer - List size standard 1 to 28: Number of standard message ID filter elements >28: Values greater than 28 are interpreted as 28. These are protected write (P) bits, which means that write access by the bits is possible only when the bit 1 \[CCE\] and bit 0 \[INIT\] of CCCR register are set to 1.
pub type LSS_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
///Field `LSE` reader - List size extended 1 to 8: Number of extended message ID filter elements >8: Values greater than 8 are interpreted as 8. These are protected write (P) bits, which means that write access by the bits is possible only when the bit 1 \[CCE\] and bit 0 \[INIT\] of CCCR register are set to 1.
pub type LSE_R = crate::FieldReader;
///Field `LSE` writer - List size extended 1 to 8: Number of extended message ID filter elements >8: Values greater than 8 are interpreted as 8. These are protected write (P) bits, which means that write access by the bits is possible only when the bit 1 \[CCE\] and bit 0 \[INIT\] of CCCR register are set to 1.
pub type LSE_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    ///Bit 0 - Reject remote frames extended These are protected write (P) bits, which means that write access by the bits is possible only when the bit 1 \[CCE\] and bit 0 \[INIT\] of CCCR register are set to 1.
    #[inline(always)]
    pub fn rrfe(&self) -> RRFE_R {
        RRFE_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Reject remote frames standard These are protected write (P) bits, which means that write access by the bits is possible only when the bit 1 \[CCE\] and bit 0 \[INIT\] of CCCR register are set to 1.
    #[inline(always)]
    pub fn rrfs(&self) -> RRFS_R {
        RRFS_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bits 2:3 - Accept non-matching frames extended Defines how received messages with 29-bit IDs that do not match any element of the filter list are treated. These are protected write (P) bits, which means that write access by the bits is possible only when the bit 1 \[CCE\] and bit 0 \[INIT\] of CCCR register are set to 1.
    #[inline(always)]
    pub fn anfe(&self) -> ANFE_R {
        ANFE_R::new(((self.bits >> 2) & 3) as u8)
    }
    ///Bits 4:5 - Accept Non-matching frames standard Defines how received messages with 11-bit IDs that do not match any element of the filter list are treated. These are protected write (P) bits, which means that write access by the bits is possible only when the bit 1 \[CCE\] and bit 0 \[INIT\] of CCCR register are set to 1.
    #[inline(always)]
    pub fn anfs(&self) -> ANFS_R {
        ANFS_R::new(((self.bits >> 4) & 3) as u8)
    }
    ///Bit 8 - FIFO 1 operation mode (overwrite or blocking) This is a protected write (P) bits, which means that write access by the bits is possible only when the bit 1 \[CCE\] and bit 0 \[INIT\] of CCCR register are set to 1.
    #[inline(always)]
    pub fn f1om(&self) -> F1OM_R {
        F1OM_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - FIFO 0 operation mode (overwrite or blocking) This is protected write (P) bits, which means that write access by the bits is possible only when the bit 1 \[CCE\] and bit 0 \[INIT\] of CCCR register are set to 1.
    #[inline(always)]
    pub fn f0om(&self) -> F0OM_R {
        F0OM_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bits 16:20 - List size standard 1 to 28: Number of standard message ID filter elements >28: Values greater than 28 are interpreted as 28. These are protected write (P) bits, which means that write access by the bits is possible only when the bit 1 \[CCE\] and bit 0 \[INIT\] of CCCR register are set to 1.
    #[inline(always)]
    pub fn lss(&self) -> LSS_R {
        LSS_R::new(((self.bits >> 16) & 0x1f) as u8)
    }
    ///Bits 24:27 - List size extended 1 to 8: Number of extended message ID filter elements >8: Values greater than 8 are interpreted as 8. These are protected write (P) bits, which means that write access by the bits is possible only when the bit 1 \[CCE\] and bit 0 \[INIT\] of CCCR register are set to 1.
    #[inline(always)]
    pub fn lse(&self) -> LSE_R {
        LSE_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RXGFC")
            .field("rrfe", &self.rrfe())
            .field("rrfs", &self.rrfs())
            .field("anfe", &self.anfe())
            .field("anfs", &self.anfs())
            .field("f1om", &self.f1om())
            .field("f0om", &self.f0om())
            .field("lss", &self.lss())
            .field("lse", &self.lse())
            .finish()
    }
}
impl W {
    ///Bit 0 - Reject remote frames extended These are protected write (P) bits, which means that write access by the bits is possible only when the bit 1 \[CCE\] and bit 0 \[INIT\] of CCCR register are set to 1.
    #[inline(always)]
    pub fn rrfe(&mut self) -> RRFE_W<RXGFCrs> {
        RRFE_W::new(self, 0)
    }
    ///Bit 1 - Reject remote frames standard These are protected write (P) bits, which means that write access by the bits is possible only when the bit 1 \[CCE\] and bit 0 \[INIT\] of CCCR register are set to 1.
    #[inline(always)]
    pub fn rrfs(&mut self) -> RRFS_W<RXGFCrs> {
        RRFS_W::new(self, 1)
    }
    ///Bits 2:3 - Accept non-matching frames extended Defines how received messages with 29-bit IDs that do not match any element of the filter list are treated. These are protected write (P) bits, which means that write access by the bits is possible only when the bit 1 \[CCE\] and bit 0 \[INIT\] of CCCR register are set to 1.
    #[inline(always)]
    pub fn anfe(&mut self) -> ANFE_W<RXGFCrs> {
        ANFE_W::new(self, 2)
    }
    ///Bits 4:5 - Accept Non-matching frames standard Defines how received messages with 11-bit IDs that do not match any element of the filter list are treated. These are protected write (P) bits, which means that write access by the bits is possible only when the bit 1 \[CCE\] and bit 0 \[INIT\] of CCCR register are set to 1.
    #[inline(always)]
    pub fn anfs(&mut self) -> ANFS_W<RXGFCrs> {
        ANFS_W::new(self, 4)
    }
    ///Bit 8 - FIFO 1 operation mode (overwrite or blocking) This is a protected write (P) bits, which means that write access by the bits is possible only when the bit 1 \[CCE\] and bit 0 \[INIT\] of CCCR register are set to 1.
    #[inline(always)]
    pub fn f1om(&mut self) -> F1OM_W<RXGFCrs> {
        F1OM_W::new(self, 8)
    }
    ///Bit 9 - FIFO 0 operation mode (overwrite or blocking) This is protected write (P) bits, which means that write access by the bits is possible only when the bit 1 \[CCE\] and bit 0 \[INIT\] of CCCR register are set to 1.
    #[inline(always)]
    pub fn f0om(&mut self) -> F0OM_W<RXGFCrs> {
        F0OM_W::new(self, 9)
    }
    ///Bits 16:20 - List size standard 1 to 28: Number of standard message ID filter elements >28: Values greater than 28 are interpreted as 28. These are protected write (P) bits, which means that write access by the bits is possible only when the bit 1 \[CCE\] and bit 0 \[INIT\] of CCCR register are set to 1.
    #[inline(always)]
    pub fn lss(&mut self) -> LSS_W<RXGFCrs> {
        LSS_W::new(self, 16)
    }
    ///Bits 24:27 - List size extended 1 to 8: Number of extended message ID filter elements >8: Values greater than 8 are interpreted as 8. These are protected write (P) bits, which means that write access by the bits is possible only when the bit 1 \[CCE\] and bit 0 \[INIT\] of CCCR register are set to 1.
    #[inline(always)]
    pub fn lse(&mut self) -> LSE_W<RXGFCrs> {
        LSE_W::new(self, 24)
    }
}
/**FDCAN global filter configuration register

You can [`read`](crate::Reg::read) this register and get [`rxgfc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rxgfc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32G491.html#FDCAN:RXGFC)*/
pub struct RXGFCrs;
impl crate::RegisterSpec for RXGFCrs {
    type Ux = u32;
}
///`read()` method returns [`rxgfc::R`](R) reader structure
impl crate::Readable for RXGFCrs {}
///`write(|w| ..)` method takes [`rxgfc::W`](W) writer structure
impl crate::Writable for RXGFCrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets RXGFC to value 0
impl crate::Resettable for RXGFCrs {}
